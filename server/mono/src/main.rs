use std::net::{IpAddr, SocketAddr};

use data::services::{
    meta::meta_service::MetaService, server_info::ServerInfo, Services, SharedServices,
};
use login::{config::LoginConfig, LoginHandler};
use moople_net::service::{
    handler::{BroadcastSender, MakeServerSessionHandler},
    session_svc::{MapleServer, SharedSessionHandle},
    BasicHandshakeGenerator, HandshakeGenerator,
};
use tokio::{net::TcpStream, task::JoinSet};

use shrooming::{FileIndex, FileSvr};

mod config;

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
        _broadcast_tx: SharedSessionHandle,
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
            "../../target/i686-pc-windows-gnu/release/moople_launchar.exe",
        ]
        .iter(),
    )?;

    FileSvr::new(file_ix).serve(addr).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    // Load configuration
    let settings = config::get_configuration().expect("Failed to load configuration");
    log::info!("{0} - Mono - {1}", settings.server_name, settings.version);

    let server_addr: IpAddr = settings.external_ip.parse()?;
    let bind_addr: IpAddr = settings.bind_ip.parse()?;

    tokio::spawn(srv_shrooming(SocketAddr::new(
        bind_addr,
        settings.shrooming_port,
    )));

    let servers = [ServerInfo::new(
        server_addr,
        settings.base_port,
        settings.server_name,
        settings.num_channels,
    )];

    // Create login server
    let handshake_gen = match settings.client_version {
        83 => BasicHandshakeGenerator::v83(),
        _ => BasicHandshakeGenerator::v95(),
    };

    let meta = Box::new(MetaService::load_from_dir("../../game_data/rbin".into())?);

    let services = Services::seeded_in_memory(servers, Box::leak(meta))
        .await?
        .as_shared();
    let (acc_id, char_id) = services.seed_acc_char().await?;
    log::info!("Created test account {acc_id} - char: {char_id}");

    let mut set = JoinSet::new();
    set.spawn(srv_login_server(
        SocketAddr::new(bind_addr, settings.base_port),
        handshake_gen.clone(),
        services.clone(),
    ));
    for ch in 0..settings.num_channels {
        set.spawn(srv_game_server(
            SocketAddr::new(bind_addr, settings.base_port + 1 + ch as u16),
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
