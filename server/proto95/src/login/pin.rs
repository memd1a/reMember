use moople_derive::MooplePacket;
use moople_packet::{maple_enum_code, packet_opcode, proto::option::MapleOption8};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes};

maple_enum_code!(
    CheckPinResult,
    u8,
    Accepted = 0,
    RegisterNewPin = 1,
    InvalidPin = 2,
    SystemError = 3,
    EnterPin = 4,
    //TODO valid?
    ResetLogin = 7
);

impl From<CheckPinResult> for CheckPinResp {
    fn from(value: CheckPinResult) -> Self {
        CheckPinResp(value)
    }
}

#[derive(MooplePacket, Debug)]
pub struct CheckPinResp( pub CheckPinResult);
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
