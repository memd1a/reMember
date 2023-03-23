use anyhow::anyhow;
use std::net::{IpAddr, SocketAddr};

use moople_packet::proto::MapleList16;

use proto95::login::world::{ChannelId, ChannelItem, WorldId, WorldInfoResp, WorldItem};

#[derive(Debug, Clone)]
pub struct ChannelInfo {
    pub ip: IpAddr,
    pub port: u16,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub ip: IpAddr,
    pub port: u16,
    pub channels: Vec<ChannelInfo>,
    pub name: String,
}

impl ChannelInfo {
    pub fn new(ip: IpAddr, port: u16, server_name: &str, id: ChannelId) -> Self {
        let print_id = id + 1;
        Self {
            ip,
            port,
            name: format!("{server_name}-{print_id}"),
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }
}

impl ServerInfo {
    pub fn new(ip: IpAddr, port: u16, name: String, channels: usize) -> Self {
        Self {
            ip,
            port,
            channels: (0..channels)
                .map(|id| ChannelInfo::new(ip, port + 1 + id as u16, &name, id as ChannelId))
                .collect(),
            name,
        }
    }

    pub fn get_channel_addr(&self, ch: ChannelId) -> anyhow::Result<SocketAddr> {
        self.channels
            .get(ch as usize)
            .map(|ch| ch.socket_addr())
            .ok_or_else(|| anyhow!("Invalid channel: {ch}"))
    }

    pub fn get_login_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }

    pub fn get_world_info(&self, world_id: WorldId) -> WorldItem {
        //TODO add some caching mechanismn so world item is not re-encoded each time
        // maybe a custom impl of encode for WorldItem
        // should look into making something like CachedPacket<Buf, WorldItem>

        let channels = self
            .channels
            .iter()
            .enumerate()
            .map(|(id, ch)| ChannelItem {
                name: ch.name.clone(),
                id: id as u8,
                adult_channel: false,
                world_id: world_id as u8,
                user_number: 0,
            })
            .collect();

        WorldItem {
            name: self.name.clone(),
            state: 1,
            event_desc: "Event!!".to_string(),
            event_exp: 100,
            event_drop_rate: 100,
            block_char_creation: false,
            channels,
            balloons: MapleList16::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ServerService {
    servers: Vec<ServerInfo>,
}

impl ServerService {
    pub fn new(servers: impl IntoIterator<Item = ServerInfo>) -> Self {
        Self {
            servers: servers.into_iter().collect(),
        }
    }

    pub fn get_server(&self, world: WorldId) -> anyhow::Result<&ServerInfo> {
        self.servers
            .get(world as usize)
            .ok_or_else(|| anyhow!("Invalid world: {world}"))
    }

    pub fn get_channel_addr(&self, world: WorldId, ch: ChannelId) -> anyhow::Result<SocketAddr> {
        self.get_server(world)?.get_channel_addr(ch)
    }

    pub fn get_world_info_packets(&self) -> Vec<WorldInfoResp> {
        self.servers
            .iter()
            .enumerate()
            .map(|(id, server)| {
                WorldInfoResp::world(id as u8, server.get_world_info(id as WorldId))
            })
            .chain(std::iter::once(WorldInfoResp::end()))
            .collect()
    }
}
