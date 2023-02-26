pub mod pin;
use moople_derive::MooplePacket;
use moople_packet::{
    maple_enum_code, packet_opcode,
    proto::{conditional::CondOption},
};

use crate::{
    recv_opcodes::RecvOpcodes,
};

pub mod account;
pub mod char;
pub mod world;

#[derive(Debug, MooplePacket)]
pub struct MachineId(pub [u8; 10]);
pub type ClientKey = [u8; 8];

pub struct LoginResultCode {}

#[derive(MooplePacket, Debug)]
pub struct CreateSecurityHandleReq;
packet_opcode!(CreateSecurityHandleReq, RecvOpcodes::CreateSecurityHandle);

maple_enum_code!(StartMode, u8, WebStart = 0, Unknown1 = 1, GameLaunching = 2);

impl StartMode {
    pub fn has_system_info(&self) -> bool {
        self != &Self::Unknown1
    }
}

#[derive(MooplePacket, Debug)]
pub struct StartModeInfo {
    start_mode: StartMode,
    #[pkt(if(field = "start_mode", cond = "StartMode::has_system_info"))]
    system_info: CondOption<SystemInfo>,
}

#[derive(MooplePacket, Debug)]
pub struct SystemInfo {
    // SupportID?
    unknown: String,
    machine_id: MachineId,
    game_room_client: u32,
    start_mode: u8,
}



#[derive(Debug, MooplePacket, Default)]
pub struct LoginResultHeader {
    pub reg: u8,
    //TODO useDay?
    pub unknown2: u32,
}

maple_enum_code!(
    LoginOpt,
    u8,
    EnableSecondPassword = 0,
    CheckSecondPassword = 1,
    NoSecondPassword1 = 2,
    NoSecondPassword2 = 3
);

/*
63, c7 => blocked for typing

*/
pub type BanReason = u8;

#[derive(Debug, MooplePacket)]
pub struct HardwareInfo {
    mac: String,
    hdd_serial_no: String,
}

#[derive(Debug, MooplePacket)]
pub struct SSOErrorLog {
    unknown1: u8,
    auth_reply_code: u32,
}
