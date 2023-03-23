use std::net::{IpAddr, SocketAddr};

use data::services::{server_info::ServerInfo, Services, SharedServices, meta::meta_service::MetaService};
use login::{config::LoginConfig, LoginHandler};
use moople_net::service::{
    handler::{MakeServerSessionHandler, BroadcastSender}, session_svc::{MapleServer, SharedSessionHandle}, BasicHandshakeGenerator,
    HandshakeGenerator,
};
use tokio::{net::TcpStream, task::JoinSet};

use shrooming::{FileSvr, FileIndex};

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
        _broadcast_tx: SharedSessionHandle
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
    world_id: u32,
    channel_id: u16,
) -> anyhow::Result<()> {
    let mut game_server = MapleServer::new(
        handshake_gen,
        game::MakeGameHandler::new(services, channel_id, world_id),
    );
    game_server.serve_tcp(addr).await?;
    Ok(())
}

async fn srv_shrooming(addr: SocketAddr) -> anyhow::Result<()> {
    let file_ix = FileIndex::build_index(
        [
            "notes.txt",
            "../../client/moople_hook/target/i686-pc-windows-gnu/release/dinput8.dll",
            "../../target/i686-pc-windows-gnu/release/moople_launchar.exe"
        ].iter()
    )?;

    FileSvr::new(file_ix)
        .serve(addr)
        .await?;

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
    const SHROOMING_PORT: u16 = 8490;
    // Change this to your external ip address
    const EXTERNAL_IP: &str = "192.168.124.1";
    // This is the bind addr, 0.0.0.0 means listen on all IPs
    const BIND_IP: &str = "0.0.0.0";

    pretty_env_logger::init();
    log::info!("{SERVER_NAME} - Mono - {VERSION}");

    
    let server_addr: IpAddr = EXTERNAL_IP.parse()?;
    let bind_addr: IpAddr = BIND_IP.parse()?;

    tokio::spawn(srv_shrooming(SocketAddr::new(bind_addr, SHROOMING_PORT)));

    let servers = [ServerInfo::new(
        server_addr,
        BASE_PORT,
        SERVER_NAME.to_string(),
        CHANNELS,
    )];

    // Create login server
    let handshake_gen = BasicHandshakeGenerator::v95();

    let meta = Box::new(MetaService::load_from_dir("../../game_data/rbin".into())?);

    let services = Services::seeded_in_memory(servers, Box::leak(meta)).await?.as_shared();
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
            0,
            ch as u16,
        ));
    }

    log::info!("Listening ...");
    while let Some(res) = set.join_next().await {
        let _ = res?;
    }

    Ok(())
}
