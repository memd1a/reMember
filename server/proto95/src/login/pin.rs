use moople_derive::MooplePacket;
use moople_packet::{mark_maple_enum, packet_opcode, proto::option::MapleOption8};
use num_enum::{TryFromPrimitive, IntoPrimitive};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum CheckPinResult {
    Accepted = 0,
    RegisterNewPin = 1,
    InvalidPin = 2,
    SystemError = 3,
    EnterPin = 4,
    //TODO valid?
    ResetLogin = 7,
}
mark_maple_enum!(CheckPinResult);

#[derive(MooplePacket, Debug)]
pub struct CheckPinResp {
    pub result: CheckPinResult,
}
packet_opcode!(CheckPinResp, SendOpcodes::CheckPinCodeResult);

#[derive(MooplePacket, Debug)]
pub struct UpdatePinResp {
    pub success: bool,
}
packet_opcode!(UpdatePinResp, SendOpcodes::UpdatePinCodeResult);

#[derive(Debug, MooplePacket)]
pub struct CheckPinData {
    //TODO: set to true in CheckPasswordResult and OnSelectWorldResult why?
    /// Somehow set to one for CLogin::OnSelectWorldResult, elsewise 0
    pub is_on_select_world_result_request: bool,
    pub pin: String,
}

#[derive(Debug, MooplePacket)]
pub struct CheckPinReq {
    pub pin: MapleOption8<CheckPinData>,
}
packet_opcode!(CheckPinReq, RecvOpcodes::CheckPinCode);

#[derive(Debug, MooplePacket)]
pub struct UpdatePinReq {
    pub pin: MapleOption8<String>,
}
packet_opcode!(UpdatePinReq, RecvOpcodes::UpdatePinCode);