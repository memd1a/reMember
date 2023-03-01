pub mod field;
use moople_derive::MooplePacket;
use moople_packet::packet_opcode;

use crate::{login::MachineId, recv_opcodes::RecvOpcodes, shared::{Vec2, char::CharacterId}};

use super::login::ClientKey;

#[derive(MooplePacket, Debug)]
pub struct MigrateInGameReq {
    pub char_id: CharacterId,
    pub machine_id: MachineId,
    pub is_gm: bool,
    pub unknown: bool,
    pub client_key: ClientKey,
}
packet_opcode!(MigrateInGameReq, RecvOpcodes::MigrateIn);

#[derive(MooplePacket, Debug)]
pub struct UpdateGMBoardReq {
    board_id: u32,
}
packet_opcode!(UpdateGMBoardReq, RecvOpcodes::UpdateGMBoard);

#[derive(MooplePacket, Debug)]
pub struct UserPortalScriptReq {
    field_key: u8,
    portal_name: String,
    pos: Vec2,
}
packet_opcode!(UserPortalScriptReq, RecvOpcodes::UserPortalScriptRequest);

#[derive(MooplePacket, Debug)]
pub struct ResetNLCPQ;
//TODO opcode name??
packet_opcode!(ResetNLCPQ, RecvOpcodes::RequireFieldObstacleStatus);
