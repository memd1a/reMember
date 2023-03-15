use moople_derive::MooplePacket;
use moople_packet::{
    maple_packet_enum, packet_opcode,
    proto::{option::MapleOption8, time::MapleTime, CondOption},
};

use crate::{
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{Gender, OptionGender},
};

use super::{BanReason, ClientKey, LoginOpt, LoginResultHeader, MachineId, StartMode};

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
}

impl AccountInfo {
    pub fn has_login_info(&self) -> bool {
        self.gender.is_set()
    }
}

#[derive(MooplePacket, Debug)]
pub struct LoginInfo {
    pub skip_pin: bool,
    pub login_opt: LoginOpt,
    pub client_key: ClientKey,
}

#[derive(MooplePacket, Debug)]
pub struct LoginAccountData {
    pub account_info: AccountInfo,
    #[pkt(if(field = "account_info", cond = "AccountInfo::has_login_info"))]
    pub login_info: CondOption<LoginInfo>,
}
#[derive(MooplePacket, Debug)]
pub struct GuestAccountInfo {
    account_id: u32,
    gender: OptionGender,
    grade_code: u8,
    sub_grade_code: u8,
    is_test_acc: bool,
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
pub struct SuccessResult {
    //TODO reg has to be either 0/1 for having an acc
    // 2/3 is some yes/no dialog
    pub hdr: LoginResultHeader,
    pub account: LoginAccountData,
}

maple_packet_enum!(
    CheckPasswordResp,
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
