use moople_derive::MooplePacket;
use moople_packet::{
    packet_opcode,
    proto::{conditional::CondOption, MapleList16, MapleList8},
};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes, shared::Vec2};

use super::StartModeInfo;

pub type WorldId = u32;
pub type WorldId16 = u16;
pub type ChannelId = u16;

#[derive(MooplePacket, Debug)]
pub struct LogoutWorldReq;
packet_opcode!(LogoutWorldReq, RecvOpcodes::LogoutWorld);

#[derive(Debug, MooplePacket)]
pub struct WorldInfoReq;
packet_opcode!(WorldInfoReq, RecvOpcodes::WorldInfoRequest);

#[derive(Debug, MooplePacket)]
pub struct WorldReq;
packet_opcode!(WorldReq, RecvOpcodes::WorldRequest);

#[derive(Debug, MooplePacket)]
pub struct WorldCheckUserLimitReq {
    pub world: WorldId16,
}
packet_opcode!(WorldCheckUserLimitReq, RecvOpcodes::CheckUserLimit);

#[derive(Debug, MooplePacket)]
pub struct WorldCheckUserLimitResp {
    pub over_user_limit: bool,
    //TODO seems like a bool
    pub populate_level: u8,
}
packet_opcode!(WorldCheckUserLimitResp, SendOpcodes::CheckUserLimitResult);

#[derive(Debug, MooplePacket)]
pub struct RecommendWorldMessage {
    world_id: WorldId,
    message: String,
}

#[derive(Debug, MooplePacket)]
pub struct RecommendWorldMessageResp {
    messages: MapleList8<RecommendWorldMessage>,
}
packet_opcode!(
    RecommendWorldMessageResp,
    SendOpcodes::RecommendWorldMessage
);

#[derive(Debug, MooplePacket)]
pub struct LastConnectedWorldResp {
    last_world: WorldId,
}
packet_opcode!(LastConnectedWorldResp, SendOpcodes::LatestConnectedWorld);

#[derive(Debug, MooplePacket)]
pub struct ChannelItem {
    pub name: String,
    pub user_number: u32,
    pub world_id: u8,
    pub id: u8,
    pub adult_channel: bool,
}

#[derive(Debug, MooplePacket)]
pub struct WorldBalloon {
    pub pos: Vec2,
    pub message: String,
}

#[derive(Debug, MooplePacket)]
pub struct WorldItem {
    pub name: String,
    pub state: u8, // 0 = normal, 1 = hot?, 2 = new
    pub event_desc: String,
    pub event_exp: u16,
    pub event_drop_rate: u16,
    pub block_char_creation: bool,
    pub channels: MapleList8<ChannelItem>,
    pub balloons: MapleList16<WorldBalloon>,
}

fn has_world_info(world_id: &u8) -> bool {
    *world_id != 0xff
}

#[derive(Debug, MooplePacket)]
pub struct WorldInfoResp {
    pub world_id: u8,
    #[pkt(if(field = "world_id", cond = "has_world_info"))]
    pub world: CondOption<WorldItem>,
}
packet_opcode!(WorldInfoResp, SendOpcodes::WorldInformation);

impl WorldInfoResp {
    pub fn end() -> Self {
        Self {
            world_id: 0xff,
            world: CondOption(None),
        }
    }

    pub fn world(id: u8, world: WorldItem) -> Self {
        Self {
            world_id: id,
            world: CondOption(Some(world)),
        }
    }
}

#[derive(MooplePacket, Debug)]
pub struct SelectWorldReq {
    pub start_mode: StartModeInfo,
    pub world_id: u8,
    pub channel_id: u8,
    // TODO: 2-5 of sa_data
    pub sa_data: u32,
}
packet_opcode!(SelectWorldReq, RecvOpcodes::SelectWorld);