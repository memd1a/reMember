use moople_derive::MooplePacket;
use moople_packet::{packet_opcode, proto::MapleList8};

use crate::{
    send_opcodes::SendOpcodes,
    shared::{char::AvatarData, movement::MovePath, FootholdId, Range2, Vec2},
};

use super::ObjectId;

pub type NpcId = u32;

#[derive(MooplePacket, Debug)]
pub struct NpcPoolPacket<T> {
    pub id: ObjectId,
    pub data: T,
}

#[derive(MooplePacket, Debug)]
pub struct NpcInitData {
    pub pos: Vec2,
    pub move_action: u8,
    pub fh: FootholdId,
    pub range_horz: Range2,
    pub enabled: bool,
}

#[derive(MooplePacket, Debug)]
pub struct NpcEnterFieldResp {
    pub id: ObjectId,
    pub template_id: NpcId,
    pub init: NpcInitData,
}
packet_opcode!(NpcEnterFieldResp, SendOpcodes::NpcEnterField);

#[derive(MooplePacket, Debug)]
pub struct NpcLeaveFieldResp {
    pub id: ObjectId,
}
packet_opcode!(NpcLeaveFieldResp, SendOpcodes::NpcLeaveField);

#[derive(MooplePacket, Debug)]
pub struct NpcImitateData {
    pub tmpl_id: NpcId,
    pub name: String,
    pub avatar_look: AvatarData,
}

#[derive(MooplePacket, Debug)]
pub struct NpcImitateDataResp {
    pub data: MapleList8<NpcImitateData>,
}
packet_opcode!(NpcImitateDataResp, SendOpcodes::ImitatedNPCData);

#[derive(MooplePacket, Debug)]
pub struct NpcUpdateLimitedDisableInfoResp {
    pub data: MapleList8<ObjectId>,
}
packet_opcode!(
    NpcUpdateLimitedDisableInfoResp,
    SendOpcodes::LimitedNPCDisableInfo
);

#[derive(MooplePacket, Debug)]
pub struct NpcChangeControllerResp {
    pub local: bool,
    pub id: ObjectId,
    //TODO only decoded if local == true
    pub tmpl_id: NpcId,
    pub init_data: NpcInitData,
}
packet_opcode!(NpcChangeControllerResp, SendOpcodes::NpcChangeController);

#[derive(MooplePacket, Debug)]
pub struct ScriptInfo {
    pub script: String,
    pub start_date: u32,
    pub end_date: u32,
}

#[derive(MooplePacket, Debug)]
pub struct ModScript {
    pub template_id: u32,
    pub script: ScriptInfo,
}

#[derive(MooplePacket, Debug)]
pub struct NpcSetScriptResp {
    pub scripts: MapleList8<ModScript>,
}
packet_opcode!(NpcSetScriptResp, SendOpcodes::NpcSetScript);

#[derive(MooplePacket, Debug)]
pub struct NpcMove {
    pub action: u8,
    pub chat: u8, //TODO correct?
    pub move_path: MovePath,
}
pub type NpcMoveResp = NpcPoolPacket<NpcMove>;
packet_opcode!(NpcMoveResp, SendOpcodes::NpcMove);

#[derive(MooplePacket, Debug)]
pub struct NpcUpdateLimitedInfo {
    pub enabled: bool,
}
pub type NpcUpdateLimitedInfoResp = NpcPoolPacket<NpcUpdateLimitedInfo>;
packet_opcode!(NpcUpdateLimitedInfoResp, SendOpcodes::NpcUpdateLimitedInfo);

#[derive(MooplePacket, Debug)]
pub struct NpcSetSpecialAction {
    pub action: String
}
pub type NpcSetSpecialActionResp = NpcPoolPacket<NpcSetSpecialAction>;
packet_opcode!(NpcSetSpecialActionResp, SendOpcodes::NpcSpecialAction);