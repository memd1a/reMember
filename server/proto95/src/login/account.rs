use moople_derive::MooplePacket;
use moople_packet::{packet_opcode, proto::{option::MapleOption8, CondOption, time::MapleTime}, maple_packet_enum};

use crate::{recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes, shared::{Gender, OptionGender}};

use super::{MachineId, LoginResultHeader, ClientKey, LoginOpt, BanReason, StartMode};

pub type AccountId = u32;


#[derive(MooplePacket, Debug)]
pub struct CheckPasswordReq {
    pub id: String,
    pub pw: String,
    pub machine_id: MachineId,
    pub game_room_client: u32,
    pub start_mode: StartMode,
    // TODO: Always 0?
    pub u1: u8,
    pub u2: u8,
    pub partner_code: u32,
}
packet_opcode!(CheckPasswordReq, RecvOpcodes::CheckPassword);

#[derive(MooplePacket, Debug)]
pub struct BlockedIp {
    pub hdr: LoginResultHeader,
    pub reason: BanReason,
    pub ban_time: MapleTime,
}

#[derive(MooplePacket, Debug)]
pub struct AccountInfo {
    account_id: u32,
    gender: OptionGender,
    grade_code: u8,
    // note: The sub_trade_code and test_acc are decoded as u16
    // on the client
    sub_grade_code: u8,
    test_acc: bool,
    country_id: u8,
    name: String,
    chat_block_reason: u8,
    purchase_exp: u8,
    chat_block_date: MapleTime,
    registration_date: MapleTime,
    num_chars: u32,
    client_key: ClientKey,
}

#[derive(MooplePacket, Debug)]
pub struct GuestAccountInfo {
    account_id: u32,
    gender: OptionGender,
    grade_code: u8,
    sub_grade_code: u8,
    test_acc: bool,
    country_id: u8,
    name: String,
    purchase_exp: u8,
    chat_block_reason: u8,
    chat_block_date: MapleTime,
    registration_date: MapleTime,
    num_chars: u32,
    guest_id_url: String,
}



#[derive(MooplePacket, Debug)]
pub struct LoginAccountExtraInfo {
    pub skip_pin: bool,
    pub login_opt: LoginOpt,
    pub client_key: ClientKey,
}

#[derive(MooplePacket, Debug)]
pub struct LoginAccountInfo {
    pub id: u32,
    pub gender: OptionGender,
    pub grade_code: u8,
    pub sub_grade_code: u8,
    pub is_test_acc: bool,
    pub country_id: u8,
    pub name: String,
    pub purchase_exp: u8,
    pub chat_block_reason: u8,
    pub chat_block_date: MapleTime,
    pub registration_date: MapleTime,
    pub num_chars: u32,
    #[pkt(if(field = "gender", cond = "OptionGender::is_set"))]
    pub extra_info: CondOption<LoginAccountExtraInfo>,
}

#[derive(MooplePacket, Debug)]
pub struct SuccessResult {
    //TODO reg has to be either 0/1 for having an acc
    // 2/3 is some yes/no dialog
    pub hdr: LoginResultHeader,
    pub account: LoginAccountInfo,
}

maple_packet_enum!(
    LoginResult,
    u8,
    Success(SuccessResult) => 0,
    BlockedIp(BlockedIp) => 2,
    IdDeleted(LoginResultHeader) => 3,
    InvalidPassword(LoginResultHeader) => 4,
    InvalidUserName(LoginResultHeader) => 5,
    SystemError(LoginResultHeader) => 6,
    AlreadyLoggedIn(LoginResultHeader) => 7,
    UnableToLoginWithIp(LoginResultHeader) => 13,
    TOS(LoginResultHeader) => 23,
    Unknown(LoginResultHeader) => 255
);

#[derive(MooplePacket, Debug)]
pub struct CheckPasswordResp {
    pub result: LoginResult,
}
packet_opcode!(CheckPasswordResp, SendOpcodes::CheckPasswordResult);



#[derive(Debug, MooplePacket)]
pub struct SetGenderReq {
    pub gender: MapleOption8<Gender>,
}
packet_opcode!(SetGenderReq, RecvOpcodes::SetGender);

impl SetGenderReq {
    pub fn set(gender: Gender) -> Self {
        Self {
            gender: Some(gender).into(),
        }
    }

    pub fn cancel() -> Self {
        Self {
            gender: None.into(),
        }
    }
}

#[derive(Debug, MooplePacket)]
pub struct SetGenderResp {
    pub gender: Gender,
    pub success: bool,
}
packet_opcode!(SetGenderResp, SendOpcodes::SetAccountResult);


#[derive(MooplePacket, Debug)]
pub struct ConfirmEULAReq {
    pub accepted: bool,
}
packet_opcode!(ConfirmEULAReq, RecvOpcodes::ConfirmEULA);

#[derive(Debug, MooplePacket)]
pub struct ConfirmEULAResp {
    pub success: bool,
}
packet_opcode!(ConfirmEULAResp, SendOpcodes::ConfirmEULAResult);
