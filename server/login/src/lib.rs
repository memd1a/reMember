use std::time::Duration;

use async_trait::async_trait;
use moople_net::{maple_router_handler, service::handler::{MapleSessionHandler, MapleServerSessionHandler}, MapleSession};
use moople_packet::{MaplePacket, MaplePacketReader, MaplePacketWriter};
use proto95::{login::account::CheckPasswordReq, send_opcodes::SendOpcodes};
use tokio::net::TcpStream;

pub struct LoginHandler {}

#[async_trait]
impl MapleSessionHandler for LoginHandler {
    type Transport = TcpStream;
    type Error = anyhow::Error;

    async fn handle_packet(
        &mut self,
        packet: MaplePacket,
        session: &mut moople_net::MapleSession<Self::Transport>,
    ) -> Result<(), Self::Error> {
        maple_router_handler!(
            handler,
            LoginHandler,
            MapleSession<TcpStream>,
            anyhow::Error,
            LoginHandler::handle_default,
            CheckPasswordReq => LoginHandler::handle_check_password,
        );

        handler(self, session, packet.into_reader()).await?;

        Ok(())
    }
}

impl LoginHandler {
    pub async fn handle_default(&mut self, pr: MaplePacketReader<'_>) -> anyhow::Result<()> {
        log::info!("Unhandled packet: {:?}", pr.into_inner());
        Ok(())
    }

    pub async fn handle_check_password(&mut self, req: CheckPasswordReq) -> anyhow::Result<()> {
        log::info!("handling check pw: {:?}", req);
        Ok(())
    }
}

impl MapleServerSessionHandler for LoginHandler {
    fn get_ping_interval() -> std::time::Duration {
        Duration::from_secs(30)
    }

    fn get_ping_packet(&mut self) -> Result<MaplePacket,Self::Error>  {
        let mut pw = MaplePacketWriter::default();
        pw.write_opcode(SendOpcodes::AliveReq);
        Ok(pw.into_packet())
    }
}