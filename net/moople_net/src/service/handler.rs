use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use futures::{Future, future};
use moople_packet::{DecodePacket, MaplePacket, MaplePacketReader, NetError};
use thiserror::Error;
use tokio::sync::mpsc;

use crate::{MapleSession, SessionTransport};

use super::resp::{IntoResponse, Response};

pub type BroadcastSender = mpsc::Sender<MaplePacket>;

#[derive(Debug, Error)]
pub enum SessionError<E> {
    #[error("Net")]
    Net(#[from] NetError),
    #[error("Session")]
    Session(E),
}

#[async_trait]
pub trait MapleSessionHandler: Sized {
    type Transport: SessionTransport;
    type Error: Debug;

    async fn handle_packet(
        &mut self,
        packet: MaplePacket,
        session: &mut MapleSession<Self::Transport>,
    ) -> Result<(), SessionError<Self::Error>>;

    async fn poll_broadcast(&mut self) -> Result<Option<MaplePacket>, Self::Error> {
        future::pending::<()>().await;
        unreachable!()
    }

    async fn finish(self, _is_migrating: bool) -> Result<(), SessionError<Self::Error>> {
        Ok(())
    }
}

#[async_trait]
pub trait MapleServerSessionHandler: MapleSessionHandler {
    fn get_ping_interval() -> Duration;
    fn get_ping_packet(&mut self) -> Result<MaplePacket, Self::Error>;
}

#[async_trait]
pub trait MakeServerSessionHandler {
    type Transport: SessionTransport;
    type Error: From<NetError> + Debug;
    type Handler: MapleServerSessionHandler<Transport = Self::Transport, Error = Self::Error>;

    async fn make_handler(
        &mut self,
        sess: &mut MapleSession<Self::Transport>,
        broadcast_tx: BroadcastSender,
    ) -> Result<Self::Handler, Self::Error>;
}

// TODO: sooner or later there should be a proper service/handler trait for this
// Prior attempts to define a service trait failed for several reasons
// 1. Unable to reuse the session to send the response after the handler was called
// 2. Lifetime 'a in DecodePacket<'a> is close to impossible to express while implementing the trait for a FnMut
// If you have better ideas how to implement this I'm completely open to this
// Also the current design is not final, It'd probably make sense to store the state
// in the session to avoid having 2 mut references, however It'd be quiet a challenge to call self methods
// on the state, cause you'd still like to have a session to send packets

pub async fn call_handler_fn<'session, F, Req, Fut, Trans, State, Resp, Err>(
    state: &'session mut State,
    session: &'session mut MapleSession<Trans>,
    mut pr: MaplePacketReader<'session>,
    mut f: F,
) -> Result<(), SessionError<Err>>
where
    Trans: SessionTransport + Send + Unpin,
    F: FnMut(&'session mut State, Req) -> Fut,
    Fut: Future<Output = Result<Resp, Err>>,
    Req: DecodePacket<'session>,
    Resp: IntoResponse,
{
    let req = Req::decode_packet(&mut pr)?;
    let resp = f(state, req)
        .await
        .map_err(SessionError::Session)?
        .into_response();
    resp.send(session).await?;
    Ok(())
}

#[macro_export]
macro_rules! maple_router_handler {
    ($name: ident, $state:ty, $session:ty, $err:ty, $default_handler:expr, $($req:ty => $handler_fn:expr),* $(,)?) => {
        async fn $name<'session>(state: &'session mut $state, session: &'session mut $session, mut pr: moople_packet::MaplePacketReader<'session>) ->  Result<(), $err> {
            let recv_op = pr.read_opcode()?;
            match recv_op {
                $(
                    <$req as moople_packet::HasOpcode>::OPCODE  => $crate::service::handler::call_handler_fn(state, session, pr, $handler_fn).await,
                )*
                _ =>   $default_handler(state, recv_op, pr).await.map_err(<$err>::Session)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::io;

    use moople_packet::{opcode::WithOpcode, MaplePacketReader, MaplePacketWriter, proto::string::FixedPacketString};

    use crate::{
        codec::{handshake::Handshake, maple_codec::PacketCodec},
        crypto::RoundKey,
        service::handler::SessionError,
        MapleSession,
    };

    pub type Req1 = WithOpcode<0, u16>;

    #[derive(Debug, Default)]
    struct State {
        req1: Req1,
    }

    impl State {
        async fn handle_req1(&mut self, req: Req1) -> anyhow::Result<()> {
            self.req1 = req;
            Ok(())
        }

        async fn handle_default(
            &mut self,
            _op: u16,
            _pr: MaplePacketReader<'_>,
        ) -> anyhow::Result<()> {
            Ok(())
        }
    }

    fn get_fake_session() -> MapleSession<std::io::Cursor<Vec<u8>>> {
        let io = std::io::Cursor::new(vec![]);
        let hshake = Handshake {
            version: 83,
            subversion: FixedPacketString::try_from("1").unwrap(),
            iv_enc: RoundKey::zero(),
            iv_dec: RoundKey::zero(),
            locale: 0,
        };
        MapleSession::from_client_handshake(io, &hshake)
    }

    #[tokio::test]
    async fn router() {
        let mut sess = get_fake_session();
        let mut state = State::default();

        let mut pw = MaplePacketWriter::default();
        pw.write_opcode(0u16);
        pw.write_u16(123);

        let pkt = pw.into_packet();

        maple_router_handler!(
            handle,
            State,
            MapleSession<io::Cursor<Vec<u8>>>,
            SessionError<anyhow::Error>,
            State::handle_default,
            Req1 => State::handle_req1,
        );

        handle(&mut state, &mut sess, pkt.into_reader())
            .await
            .unwrap();

        assert_eq!(state.req1.0, 123);
    }
}
