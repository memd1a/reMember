use std::{fmt::Debug, io, marker::PhantomData, time::Duration};

use futures::{Stream, StreamExt};
use moople_packet::NetError;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_util::sync::CancellationToken;

use crate::{codec::handshake::Handshake, MapleSession, service::handler::SessionHandleResult};

use super::{
    framed_pipe::{framed_pipe, FramedPipeReceiver, FramedPipeSender},
    handler::{MakeServerSessionHandler, MapleServerSessionHandler, MapleSessionHandler},
    HandshakeGenerator, packet_buffer::PacketBuffer,
};

#[derive(Debug, Clone)]
pub struct SharedSessionHandle {
    pub ct: CancellationToken,
    pub tx: FramedPipeSender,
}

impl SharedSessionHandle {
    pub fn try_send_buf(&mut self, pkt_buf: &PacketBuffer) -> anyhow::Result<()> {
        Ok(self.tx.try_send_all(pkt_buf.packets())?)
    }

    pub fn try_send(&mut self, item: &[u8]) -> anyhow::Result<()> {
        Ok(self.tx.try_send(item)?)
    }
}

impl SharedSessionHandle {
    pub fn new() -> (Self, FramedPipeReceiver) {
        let (tx, rx) = framed_pipe(8 * 1024, 128);
        (
            Self {
                ct: CancellationToken::new(),
                tx,
            },
            rx,
        )
    }
}

#[derive(Debug)]
pub struct MapleSessionHandle<H: MapleSessionHandler> {
    pub handle: tokio::task::JoinHandle<Result<(), H::Error>>,
    _handler: PhantomData<H>,
}

impl<H> MapleSessionHandle<H>
where
    H: MapleSessionHandler + Send,
{
    pub fn is_running(&self) -> bool {
        !self.handle.is_finished()
    }

    async fn exec_server_session(
        mut session: MapleSession<H::Transport>,
        mut handler: H,
        session_handle: SharedSessionHandle,
        mut session_rx: FramedPipeReceiver,
    ) -> Result<(), H::Error>
    where
        H: MapleServerSessionHandler,
        H::Transport: Unpin,
    {
        let mut ping_interval = tokio::time::interval(H::get_ping_interval());
        ping_interval.tick().await;
        let mut pending_ping = false;

        loop {
            //TODO might need some micro-optimization to ensure no future gets stalled
            tokio::select! {
                biased;
                // Handle next incoming packet
                p = session.read_packet() => {
                    let p = p?;
                    let res = handler.handle_packet(p, &mut session).await?;
                    // Handle special results here
                    match res {
                        SessionHandleResult::Migrate => {
                            log::info!("Session migrated");
                            handler.finish(true).await?;
                            // Socket has to be kept open cause the client doesn't support
                            // reading a packet when the socket is closed
                            // TODO: make this configurable
                            tokio::time::sleep(Duration::from_millis(7500)).await;
                            break;
                        },
                        SessionHandleResult::Pong => {
                            // TODO handle this here
                            log::info!("Pong handle");
                            pending_ping = false;
                        },
                        SessionHandleResult::Ok => ()
                    }
                },
                _ = ping_interval.tick() => {
                    if pending_ping {
                        log::error!("Ping Timeout");
                        break;
                    }
                    log::info!("Sending ping...");
                    pending_ping = true;
                    let ping_packet = handler.get_ping_packet()?;
                    session.send_raw_packet(&ping_packet.data).await?;
                },
                //Handle external Session packets
                p = session_rx.next() => {
                    // note tx is never dropped, so there'll be always a packet here
                    let p = p.expect("Session packet");
                    session.send_raw_packet(&p).await?;
                },
                p = handler.poll_broadcast() => {
                    let p = p?.expect("Must contain packet");
                    session.send_raw_packet(&p.data).await?;
                },
                _ = session_handle.ct.cancelled() => {
                    break;
                },

            };
        }

        session.close().await?;

        // Normal cancellation by timeout or cancellation
        // TODO: handle panic and gracefully shutdown the session(for example write data to db and other stuff)
        Ok(())
    }

    pub fn spawn_server_session<M>(
        io: M::Transport,
        mut mk: M,
        handshake: Handshake,
    ) -> Result<Self, M::Error>
    where
        M: MakeServerSessionHandler<Handler = H, Transport = H::Transport, Error = H::Error>
            + Send
            + 'static,
        H: MapleServerSessionHandler + Send + 'static,
        H::Transport: Unpin + Send + 'static,
        H::Error: Send + 'static,
    {
        let handle = tokio::spawn(async move {
            let res = async move {
                let mut session = MapleSession::initialize_server_session(io, handshake).await?;

                let (sess_handle, sess_rx) = SharedSessionHandle::new();
                let handler = mk
                    .make_handler(&mut session, sess_handle.clone())
                    .await?;

                let res = Self::exec_server_session(session, handler, sess_handle, sess_rx).await;
                if let Err(ref err) = res {
                    log::info!("Session exited with error: {:?}", err);
                }

                Ok(())
            };

            let res = res.await;
            if let Err(ref err) = res {
                log::error!("Session error: {:?}", err);
            }

            res
        });

        Ok(MapleSessionHandle {
            handle,
            _handler: PhantomData,
        })
    }
}

#[derive(Debug)]
pub struct MapleServer<MH, H>
where
    MH: MakeServerSessionHandler,
{
    handshake_gen: H,
    make_handler: MH,
    handles: Vec<MapleSessionHandle<MH::Handler>>,
}

impl<MH, H> MapleServer<MH, H>
where
    H: HandshakeGenerator,
    MH: MakeServerSessionHandler,
    MH::Handler: Send,
{
    pub fn new(handshake_gen: H, make_handler: MH) -> Self {
        Self {
            handshake_gen,
            make_handler,
            handles: Vec::new(),
        }
    }

    fn remove_closed_handles(&mut self) {
        self.handles.retain(|handle| handle.is_running());
    }

    fn handle_incoming(&mut self, io: MH::Transport) -> Result<(), MH::Error>
    where
        MH: Send + Clone + 'static,
        MH::Error: From<io::Error> + Send + 'static,
        MH::Handler: Send + 'static,
        MH::Transport: Send + Unpin + 'static,
    {
        let handshake = self.handshake_gen.generate_handshake();
        let handle =
            MapleSessionHandle::spawn_server_session(io, self.make_handler.clone(), handshake)?;
        // TODO: there should be an upper limit for active connections
        // cleaning closed connection should operate on Vec<Option<Handle>> probably
        // so a new conneciton just has to find a gap
        // If the last insert/clean index is stored performance should be good
        self.remove_closed_handles();
        self.handles.push(handle);

        Ok(())
    }

    pub async fn run<S>(&mut self, mut io: S) -> Result<(), MH::Error>
    where
        MH: Send + Clone + 'static,
        MH::Error: From<io::Error> + Send + 'static,
        MH::Handler: Send + 'static,
        MH::Transport: Send + Unpin + 'static,
        S: Stream<Item = std::io::Result<MH::Transport>> + Unpin,
    {
        while let Some(io) = io.next().await {
            let io = io.map_err(NetError::IO)?;
            self.handle_incoming(io)?;
        }

        Ok(())
    }
}

impl<MH, H> MapleServer<MH, H>
where
    H: HandshakeGenerator,
    MH::Error: From<io::Error> + Send + 'static,
    MH::Handler: Send + 'static,
    MH::Transport: Send + Unpin + 'static,
    MH: MakeServerSessionHandler<Transport = TcpStream> + Send + Clone + 'static,
    MH::Error: From<io::Error> + Send + 'static,
{
    pub async fn serve_tcp(
        &mut self,
        addr: impl ToSocketAddrs,
    ) -> Result<(), MH::Error> {
        let listener = TcpListener::bind(addr).await.map_err(NetError::IO)?;

        loop {
            let (io, _) = listener.accept().await.map_err(NetError::IO)?;
            self.handle_incoming(io)?;
        }
    }
}
