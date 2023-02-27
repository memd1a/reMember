use std::net::SocketAddr;

use login::LoginHandler;
use moople_net::service::{handler::MakeServerSessionHandler, BasicHandshakeGenerator, session_svc::MapleServer};
use tokio::net::TcpStream;

#[derive(Clone, Debug)]
pub struct Shared;

#[derive(Debug, Clone)]
pub struct MakeLoginHandler {
    shared: Shared,
}

#[async_trait::async_trait]
impl MakeServerSessionHandler for MakeLoginHandler {
    type Transport = TcpStream;

    type Error = anyhow::Error;

    type Handler = LoginHandler;

    async fn make_handler(
        &mut self,
        sess: &mut moople_net::MapleSession<Self::Transport>,
    ) -> Result<Self::Handler, Self::Error> {
        //TODO pass it to the login handler
        let _svcs = self.shared.clone();
        Ok(LoginHandler{})
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let login_addr: SocketAddr = "0.0.0.0:8484".parse().unwrap();
    // Create login server
    let handshake_gen = BasicHandshakeGenerator::new(95, "1".to_string(), 8);

    let mut login_server = MapleServer::new(handshake_gen, MakeLoginHandler{shared: Shared});

    log::info!("Listening for login server on port: {}", login_addr.port());
    login_server.serve_tcp(login_addr).await?;

    Ok(())
}
