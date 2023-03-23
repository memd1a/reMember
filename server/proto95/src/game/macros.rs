use moople_derive::MooplePacket;
use moople_packet::{proto::MapleList8, packet_opcode};

use crate::{id::SkillId, send_opcodes::SendOpcodes};

#[derive(MooplePacket, Debug)]
pub struct SingleMacro {
    pub name: String,
    pub mute: bool,
    pub skills: [SkillId; 3]
}

pub type MacroSysData = MapleList8<SingleMacro>;

#[derive(MooplePacket, Debug)]
pub struct MacroSysDataInitResp {
    pub data: MacroSysData
    
}
packet_opcode!(MacroSysDataInitResp, SendOpcodes::MacroSysDataInit);