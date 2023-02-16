use futures::Future;
use moople_packet::{DecodePacket, MaplePacketReader};

use crate::{MapleSession, SessionTransport};

use super::resp::{IntoResponse, Response};

pub async fn call_handler_fn<'a, F, Req, Fut, Trans, State, Resp>(
    state: &'a mut State,
    session: &'a mut MapleSession<Trans>,
    pr: &'a mut MaplePacketReader<'a>,
    mut f: F,
) -> anyhow::Result<()>
where
    Trans: SessionTransport + Send + Unpin,
    F: FnMut(&'a mut State, Req) -> Fut,
    Fut: Future<Output = anyhow::Result<Resp>>,
    Req: DecodePacket<'a>,
    Resp: IntoResponse,
{
    let req = Req::decode_packet(pr)?;
    let resp = f(state, req).await?.into_response();
    resp.send(session).await?;
    Ok(())
}

#[macro_export]
macro_rules! maple_router_handler {
    ($pr:ident, $state:ident, $session:ident, $default_handler:expr, $($req:ty => $handler_fn:expr,)*) => {
        async {
            let recv_op = $pr.read_opcode()?;
            match recv_op {
                $(
                    <$req as moople_packet::HasOpcode>::OPCODE  => { $crate::service::handler::call_handler_fn::<'_, _, $req, _, _, _, _>($state, $session, &mut $pr, $handler_fn).await? }
                ),*
                _ =>   { $default_handler($state, &mut $pr).await? }
            }
            anyhow::Ok(())
        }
    };
}

#[cfg(test)]
mod tests {
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

        async fn handle_default(&mut self, _pr: &mut MaplePacketReader<'_>) -> anyhow::Result<()> {
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
        let mut pr = pkt.into_reader();

        let s = &mut state;
        let ss = &mut sess;
        
        maple_router_handler!(
            pr,
            s,
            ss,
            State::handle_default,
            Req1 => State::handle_req1,
        )
        .await
        .unwrap();

        assert_eq!(state.req1.0, 123);
    }
}
