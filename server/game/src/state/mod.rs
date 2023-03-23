use std::net::IpAddr;

use data::services::{
    field::FieldJoinHandle,
    session::{session_data::OwnedMoopleSession, session_set::SharedSessionDataRef, ClientKey},
};
use moople_net::service::packet_buffer::PacketBuffer;
use proto95::{
    login::world::{ChannelId, WorldId},
    shared::{char::CharStatPartial, FootholdId, Vec2},
};

use crate::repl::GameRepl;

use self::char::PartialCharStats;

pub mod char;

pub struct SessionState {
    pub session: OwnedMoopleSession,
    pub session_handle: SharedSessionDataRef,
    pub peer_addr: IpAddr,
    pub world_id: WorldId,
    pub channel_id: ChannelId,
    pub client_key: ClientKey,
    pub char_stats: CharStatPartial,
}

pub struct CharState {
    pub pos: Vec2,
    pub fh: FootholdId,
    pub field: FieldJoinHandle,
}

pub struct GameState {
    pub session: SessionState,
    pub repl: GameRepl,
    pub packet_buf: PacketBuffer,
    pub char: CharState,
    pub char_update: PartialCharStats,
}

impl GameState {
    
}
