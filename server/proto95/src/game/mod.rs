pub mod reactor;
pub mod pet;
pub mod npc;
pub mod chat;
pub mod drop;
pub mod field;
pub mod friend;
pub mod keymaps;
pub mod macros;
pub mod mob;
pub mod user;
use moople_derive::MooplePacket;
use moople_packet::{maple_packet_enum, packet_opcode, proto::time::Ticks};

use crate::{
    id::job_id::JobId,
    login::MachineId,
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{char::CharacterId, Gender, ServerSocketAddr, Vec2},
};

use super::login::ClientKey;

pub type ObjectId = u32;

#[derive(MooplePacket, Debug)]
pub struct CharacterInfoReq {
    pub ticks: Ticks,
    pub char_id: CharacterId,
    pub pet_info: bool,
}
packet_opcode!(CharacterInfoReq, RecvOpcodes::UserCharacterInfoRequest);

#[derive(MooplePacket, Debug)]
pub struct CharacterInfoResp {
    pub char_id: CharacterId,
    pub level: u8,
    pub job: JobId,
}
packet_opcode!(CharacterInfoResp, SendOpcodes::CharacterInfo);

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
pub struct TransferChannelReq {
    pub channel_id: u8,
    pub ticks: Ticks,
}
packet_opcode!(TransferChannelReq, RecvOpcodes::UserTransferChannelRequest);

#[derive(MooplePacket, Debug)]
pub struct MigrateCommandResp {
    pub unknown: bool, //always true?
    pub addr: ServerSocketAddr,
}
packet_opcode!(MigrateCommandResp, SendOpcodes::MigrateCommand);

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

#[derive(MooplePacket, Debug)]
pub struct CtxSetGenderResp {
    pub gender: Gender,
}
packet_opcode!(CtxSetGenderResp, SendOpcodes::SetGender);

#[derive(MooplePacket, Debug)]
pub struct ClaimSvrStatusChangedResp {
    pub connected: bool,
}
packet_opcode!(
    ClaimSvrStatusChangedResp,
    SendOpcodes::ClaimSvrStatusChanged
);

#[derive(MooplePacket, Debug)]
pub struct ServerMessage {
    pub flag: bool,
    pub msg: String,
}

maple_packet_enum!(
    BroadcastMessageResp,
    u8,
    ServerMessage(ServerMessage) => 4,
    PinkMessage(String) => 5,
);
packet_opcode!(BroadcastMessageResp, SendOpcodes::BroadcastMsg);
