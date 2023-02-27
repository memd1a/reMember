use std::{io, marker::PhantomData};

use futures::{channel::mpsc, SinkExt, Stream, StreamExt};
use moople_packet::{MaplePacket, NetError};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_util::sync::CancellationToken;

use crate::{codec::handshake::Handshake, MapleSession};

use super::{
    handler::{MakeServerSessionHandler, MapleServerSessionHandler, MapleSessionHandler},
    HandshakeGenerator,
};

#[derive(Debug)]
pub struct MapleSessionHandle<H: MapleSessionHandler> {
    pub broadcast_tx: mpsc::Sender<MaplePacket>,
    pub ct: CancellationToken,
    pub handle: tokio::task::JoinHandle<Result<(), H::Error>>,
    _handler: PhantomData<H>,
}

impl<H> MapleSessionHandle<H>
where
    H: MapleSessionHandler,
{
    pub fn cancel(&mut self) {
        self.ct.cancel();
    }

    pub fn is_running(&self) -> bool {
        !self.handle.is_finished()
    }

    async fn exec_server_session(
        mut session: MapleSession<H::Transport>,
        mut handler: H,
        mut broadcast_rx: mpsc::Receiver<MaplePacket>,
        ct: CancellationToken,
    ) -> Result<(), H::Error>
    where
        H: MapleServerSessionHandler,
        H::Transport: Unpin,
    {
        let mut ping_interval = tokio::time::interval(H::get_ping_interval());
        ping_interval.tick().await;

        loop {
            //TODO might need some micro-optimization to ensure no future gets stalled
            tokio::select! {
                biased;
                // Handle next incoming packet
                p = session.read_packet() => {
                    match p {
                        Ok(p) => handler.handle_packet(p, &mut session).await?,
                        Err(NetError::IO(err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                            log::info!("Client disconnected");
                            break;
                        },
                        Err(err) => { return Err(err.into()); }
                    };
                    /*let p = p?;
                    handler.handle_packet(p, &mut session).await?;*/
                },
                _ = ping_interval.tick() => {
                    let ping_packet = handler.get_ping_packet()?;
                    log::info!("Sending ping packet: {:?}", ping_packet.data);
                    session.codec.send(ping_packet).await?;
                },
                //Handle broadcast packets
                p = broadcast_rx.next() => {
                    // note tx is never dropped, so there'll be always a packet here
                    let p = p.unwrap();
                    session.codec.send(p).await?;
                },
                _ = ct.cancelled() => {
                    break;
                },

            };
        }

        session.shutdown().await?;

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
        let (broadcast_tx, broadcast_rx) = mpsc::channel(16);
        let ct = CancellationToken::new();
        let ct_session = ct.clone();

        let handle = tokio::spawn(async move {
            let res = async move {
                let mut session = MapleSession::initialize_server_session(io, &handshake).await?;
                let handler = mk.make_handler(&mut session).await?;
                let res =
                    Self::exec_server_session(session, handler, broadcast_rx, ct_session).await;
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
            broadcast_tx,
            ct,
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
            let io = io?;
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
    pub async fn serve_tcp(&mut self, addr: impl ToSocketAddrs) -> Result<(), MH::Error> {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (io, _) = listener.accept().await?;
            self.handle_incoming(io)?;
        }
    }
}
