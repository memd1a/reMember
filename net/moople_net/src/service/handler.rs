use std::{fmt::Debug, time::Duration};

use async_trait::async_trait;
use futures::Future;
use moople_packet::{DecodePacket, MaplePacket, MaplePacketReader, NetError};

use crate::{MapleSession, SessionTransport};

use super::resp::{IntoResponse, Response};

#[async_trait]
pub trait MapleSessionHandler {
    type Transport: SessionTransport;
    type Error: From<NetError> + Debug;

    async fn handle_packet(
        &mut self,
        packet: MaplePacket,
        session: &mut MapleSession<Self::Transport>,
    ) -> Result<(), Self::Error>;
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

pub async fn call_handler_fn<'session, F, Req, Fut, Trans, State, Resp>(
    state: &'session mut State,
    session: &'session mut MapleSession<Trans>,
    mut pr: MaplePacketReader<'session>,
    mut f: F,
) -> anyhow::Result<()>
where
    Trans: SessionTransport + Send + Unpin,
    F: FnMut(&'session mut State, Req) -> Fut,
    Fut: Future<Output = anyhow::Result<Resp>>,
    Req: DecodePacket<'session>,
    Resp: IntoResponse,
{
    let req = Req::decode_packet(&mut pr)?;
    let resp = f(state, req).await?.into_response();
    resp.send(session).await?;
    Ok(())
}

#[macro_export]
macro_rules! maple_router_handler {
    ($name: ident, $state:ty, $session:ty, $err:ty, $default_handler:expr, $($req:ty => $handler_fn:expr,)*) => {
        async fn $name<'session>(state: &'session mut $state, session: &'session mut $session, mut pr: moople_packet::MaplePacketReader<'session>) ->  Result<(), $err> {
            let recv_op = pr.read_opcode()?;
            match recv_op {
                $(
                    <$req as moople_packet::HasOpcode>::OPCODE  => $crate::service::handler::call_handler_fn(state, session, pr, $handler_fn).await,
                ),*
                _ =>   $default_handler(state, pr).await
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::io;

    use moople_packet::{
        opcode::HasOpcode, proto::wrapped::MapleWrapped, MaplePacketReader, MaplePacketWriter,
    };
    use tokio_util::codec::Framed;

    use crate::{
        codec::{handshake::Handshake, maple_codec::MapleCodec},
        crypto::RoundKey,
        MapleSession,
    };

    #[derive(Debug, Default)]
    struct Req1(u16);
    impl MapleWrapped for Req1 {
        type Inner = u16;

        fn maple_into_inner(&self) -> Self::Inner {
            self.0
        }

        fn maple_from(v: Self::Inner) -> Self {
            Self(v)
        }
    }

    impl HasOpcode for Req1 {
        type OP = u16;

        const OPCODE: Self::OP = 0;
    }

    #[derive(Debug, Default)]
    struct State {
        req1: Req1,
    }

    impl State {
        async fn handle_req1(&mut self, req: Req1) -> anyhow::Result<()> {
            self.req1 = req;
            Ok(())
        }

        async fn handle_default(&mut self, _pr: MaplePacketReader<'_>) -> anyhow::Result<()> {
            Ok(())
        }
    }

    fn get_fake_session() -> MapleSession<std::io::Cursor<Vec<u8>>> {
        let io = std::io::Cursor::new(vec![]);
        let codec = MapleCodec::client_from_handshake(&Handshake {
            version: 83,
            subversion: "1".to_string(),
            iv_enc: RoundKey::zero(),
            iv_dec: RoundKey::zero(),
            locale: 0,
        });
        MapleSession::new(Framed::new(io, codec))
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
            anyhow::Error,
            State::handle_default,
            Req1 => State::handle_req1,
        );

        handle(&mut state, &mut sess, pkt.into_reader())
            .await
            .unwrap();

        assert_eq!(state.req1.0, 123);
    }
}
