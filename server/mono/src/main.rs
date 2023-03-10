use std::net::{IpAddr, SocketAddr};

use login::{config::LoginConfig, LoginHandler};
use moople_net::service::{
    handler::MakeServerSessionHandler, session_svc::MapleServer, BasicHandshakeGenerator,
    HandshakeGenerator,
};
use services::{server_info::ServerInfo, Services, SharedServices};
use tokio::{net::TcpStream, task::JoinSet};

static LOGIN_CFG: &LoginConfig = &LoginConfig {
    enable_pic: true,
    enable_pin: false,
};

#[derive(Clone, Debug)]
pub struct Shared;

#[derive(Debug, Clone)]
pub struct MakeLoginHandler {
    services: SharedServices,
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
        Ok(LoginHandler::new(
            self.services.clone(),
            LOGIN_CFG,
            sess.peer_addr()?.ip(),
        ))
    }
}



async fn srv_login_server(
    addr: impl tokio::net::ToSocketAddrs,
    handshake_gen: impl HandshakeGenerator,
    services: SharedServices,
) -> anyhow::Result<()> {
    let mut login_server = MapleServer::new(handshake_gen, MakeLoginHandler { services });
    login_server.serve_tcp(addr).await?;
    Ok(())
}

async fn srv_game_server(
    addr: impl tokio::net::ToSocketAddrs,
    handshake_gen: impl HandshakeGenerator,
    services: SharedServices,
) -> anyhow::Result<()> {
    let mut game_server = MapleServer::new(handshake_gen, game::MakeGameHandler::new(services));
    game_server.serve_tcp(addr).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Config
    const VERSION: &str = "v0.1a";
    const SERVER_NAME: &str = "reMember";
    const WORLDS: usize = 1;
    const CHANNELS: usize = 3;
    // Base port, this will be the port of the Login Server
    // Channel ports will be BASE_PORT + 1 + ch
    const BASE_PORT: u16 = 8484;
    // Change this to your external ip address
    const EXTERNAL_IP: &str = "192.168.124.1";
    // This is the bind addr, 0.0.0.0 means listen on all IPs
    const BIND_IP: &str = "0.0.0.0";

    pretty_env_logger::init();
    log::info!("{SERVER_NAME} - Mono - {VERSION}");

    let server_addr: IpAddr = EXTERNAL_IP.parse()?;
    let bind_addr: IpAddr = BIND_IP.parse()?;

    let servers = [ServerInfo::new(
        server_addr,
        BASE_PORT,
        SERVER_NAME.to_string(),
        CHANNELS,
    )];

    // Create login server
    let handshake_gen = BasicHandshakeGenerator::new(95, "1".to_string(), 8);

    let services = Services::seeded_in_memory(servers).await?.as_shared();
    let (acc_id, char_id) = services.seed_acc_char().await?;
    log::info!("Created test account {acc_id} - char: {char_id}");

    let mut set = JoinSet::new();
    set.spawn(srv_login_server(
        SocketAddr::new(bind_addr, BASE_PORT),
        handshake_gen.clone(),
        services.clone(),
    ));
    for ch in 0..CHANNELS {
        set.spawn(srv_game_server(
            SocketAddr::new(bind_addr, BASE_PORT + 1 + ch as u16),
            handshake_gen.clone(),
            services.clone(),
        ));
    }

    log::info!("Listening ...");
    while let Some(res) = set.join_next().await {
        let _ = res?;
    }

    Ok(())
}
