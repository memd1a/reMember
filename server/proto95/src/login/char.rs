use moople_derive::MooplePacket;
use moople_packet::{
    maple_packet_enum, mark_maple_enum, packet_opcode,
    proto::{option::MapleOption8, MapleList8},
};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    id::{
        job_id::{JobGroup, SubJob},
        FaceId, HairId, ItemId,
    },
    recv_opcodes::RecvOpcodes,
    send_opcodes::SendOpcodes,
    shared::{Gender, char::{CharStat, AvatarData}, ServerAddr},
};

use super::{account::AccountId, world::WorldId, StartModeInfo, LoginOpt, HardwareInfo};

type CharacterId = u32;

#[derive(MooplePacket, Debug)]
pub struct ViewAllCharFlagSet {
    pub set: bool,
}
packet_opcode!(ViewAllCharFlagSet, RecvOpcodes::VACFlagSet);

#[derive(MooplePacket, Debug)]
pub struct MigrateStageInfo {
    pub addr: ServerAddr,
    pub port: u16,
    pub char_id: CharacterId,
    pub premium: bool,
    pub premium_arg: u32,
}

maple_packet_enum!(
    SelectCharResult,
    u8,
    Success(MigrateStageInfo) => 0,
    //TODO add the rest
);

#[derive(MooplePacket, Debug)]
pub struct SelectCharResp {
    //TODO: use enums
    pub error_code: u8,
    //TODO add all options
    pub result: SelectCharResult,
}
packet_opcode!(SelectCharResp, SendOpcodes::SelectCharacterResult);

//TODO how does this work? must use prestored world i guess
#[derive(MooplePacket, Debug)]
pub struct ViewAllCharReq {
    start_mode: StartModeInfo,
}
packet_opcode!(ViewAllCharReq, RecvOpcodes::ViewAllChar);

maple_packet_enum!(
    ViewAllCharResp,
    u8,
    Success(ViewAllCharList) => 0,
    Prepare(ViewAllCharPrepare) => 1,
    Reset(()) => 2,
    Error3(ViewAllCharCustomError) => 3,
    Error4(()) => 4,
    Error5(()) => 5,
    Error6(ViewAllCharCustomError) => 6,
    Error7(ViewAllCharCustomError) => 7
);
packet_opcode!(ViewAllCharResp, SendOpcodes::ViewAllCharResult);

maple_packet_enum!(
    SelectWorldResp,
    u8,
    Success(SelectWorldCharList) => 0,
    Err(()) => 1 //TODO add more errors
);
packet_opcode!(SelectWorldResp, SendOpcodes::SelectWorldResult);

maple_packet_enum!(
    CreateCharResp,
    u8,
    Success(ViewChar) => 0,
    Timeout(()) => 0xa,
    SystemError(()) => 0x1a,
    InvalidCharName(()) => 0x1e,
    //TODO more errors?
);
packet_opcode!(CreateCharResp, SendOpcodes::CreateNewCharacterResult);

#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum DeleteCharResult {
    //TODO verify
    Success = 0,
    DBFail = 6,
    UnknownErr = 9,
    Timeout = 0xA,
    InvalidBirthday = 0x12,
    InvalidPic = 0x14,
    ErrGuildMaster = 0x16,
    ErrPendingWedding = 0x18,
    ErrPendingWorldTransfer = 0x1A,
    ErrHasFamily = 0x1D,
}
mark_maple_enum!(DeleteCharResult);

#[derive(MooplePacket, Debug)]
pub struct DeleteCharResp {
    pub char_id: CharacterId,
    pub result: DeleteCharResult,
}
packet_opcode!(DeleteCharResp, SendOpcodes::DeleteCharacterResult);

#[derive(MooplePacket, Debug)]
pub struct DeleteCharReq {
    pub pic: String,
    pub char_id: CharacterId,
}
packet_opcode!(DeleteCharReq, RecvOpcodes::DeleteCharacter);

#[derive(MooplePacket, Debug)]
pub struct EnableSecondPasswordResp {
    pub success: bool,
    // TODO <= 0x17, some error code like others
    pub result: u8,
}
packet_opcode!(EnableSecondPasswordResp, SendOpcodes::EnableSPWResult);

#[derive(MooplePacket, Debug)]
pub struct CheckSecondPasswordResp {
    pub u1: u8, // Todo: Unused code??
}
packet_opcode!(CheckSecondPasswordResp, SendOpcodes::CheckSPWResult);

#[derive(Debug, MooplePacket)]
pub struct ExtraCharInfoResp {
    pub acc_id: AccountId,
    pub no_extra_char: bool,
}
packet_opcode!(ExtraCharInfoResp, SendOpcodes::CheckExtraCharInfoResult);

#[derive(MooplePacket, Debug)]
pub struct ViewChar {
    pub stats: CharStat,
    pub avatar_data: AvatarData,
}

#[derive(MooplePacket, Debug)]
pub struct CharRankInfo {
    pub world_rank: u32,
    pub rank_move: u32, /* gap */
    pub job_rank: u32,
    pub job_rank_mode: u32, /* gap */
}

#[derive(MooplePacket, Debug)]
pub struct ViewCharWithRank {
    pub view_char: ViewChar,
    pub u1: u8, //VAC?
    pub rank_info: MapleOption8<CharRankInfo>,
}

#[derive(MooplePacket, Debug)]
pub struct SelectWorldCharList {
    pub characters: MapleList8<ViewCharWithRank>,
    pub login_opt: LoginOpt,
    pub slot_count: u32,
    pub buy_char_count: u32,
}

#[derive(MooplePacket, Debug)]
pub struct ViewAllCharList {
    pub world_id: u8,
    pub characters: MapleList8<ViewChar>,
    pub login_opt: LoginOpt,
}

#[derive(MooplePacket, Debug)]
pub struct ViewAllCharCustomError {
    pub msg: MapleOption8<String>,
}

#[derive(MooplePacket, Debug)]
pub struct ViewAllCharPrepare {
    pub count_related_servers: u32,
    pub count_chars: u32,
}

#[derive(MooplePacket, Debug)]
pub struct CharacterRankData {
    world_rank: u32,
    world_rank_gap: u32,
    job_rank: u32,
    job_rank_gap: u32,
}



#[derive(MooplePacket, Debug)]
pub struct ViewExtraInfo {
    hardware_id: String,
    machine_id: [u8; 0x10],
    game_room_client: u32,
    start_mode: u8,
}

#[derive(MooplePacket, Debug)]
pub struct ViewAllCharRequest {
    extra_info: MapleOption8<ViewExtraInfo>,
}

#[derive(MooplePacket, Debug)]
pub struct SelectCharEnablePicReq {
    pub unknown1: u8, //Always 1 ?
    pub char_id: CharacterId,
    pub hw_info: HardwareInfo,
    pub pic: String,
}
packet_opcode!(SelectCharEnablePicReq, RecvOpcodes::EnableSPWRequest);

#[derive(MooplePacket, Debug)]
pub struct SelectCharCheckPicReq {
    pub pic: String,
    pub char_id: CharacterId,
    pub hw_info: HardwareInfo,
}
packet_opcode!(SelectCharCheckPicReq, RecvOpcodes::CheckSPWRequest);

#[derive(MooplePacket, Debug)]
pub struct SelectCharReq {
    pub char_id: CharacterId,
    pub hw_info: HardwareInfo,
}
packet_opcode!(SelectCharReq, RecvOpcodes::SelectCharacter);

// Login Opt 0  == Enable Second Password
#[derive(MooplePacket, Debug)]
pub struct SelectCharEnablePicVac {
    pub unknown1: u8, //Always 1 ?
    pub char_id: CharacterId,
    pub world_id: WorldId,
    pub hw_info: HardwareInfo,
    pub pic: String,
}
packet_opcode!(SelectCharEnablePicVac, RecvOpcodes::EnableSPWRequestByVAC);

// Login Opt 1  == Check Second Password
#[derive(MooplePacket, Debug)]
pub struct SelectCharCheckPicVac {
    pub pic: String,
    pub char_id: CharacterId,
    pub world_id: WorldId,
    pub hw_info: HardwareInfo,
}
packet_opcode!(SelectCharCheckPicVac, RecvOpcodes::CheckSPWRequestByVAC);

// Login Opt 2/3
#[derive(MooplePacket, Debug)]
pub struct SelectCharReqVac {
    char_id: CharacterId,
    world_id: WorldId,
    hw_info: HardwareInfo,
}
packet_opcode!(SelectCharReqVac, RecvOpcodes::SelectCharacterByVAC);

#[derive(MooplePacket, Debug)]
pub struct CharStarterSet {
    pub face: FaceId,
    pub hair: HairId,
    pub hair_color: u32,
    pub skin_color: u32,
    pub top: ItemId,
    pub bottom: ItemId,
    pub shoes: ItemId,
    pub weapon: ItemId,
}

#[derive(MooplePacket, Debug)]
pub struct CreateCharReq {
    pub name: String,
    pub job: JobGroup,
    pub sub_job: SubJob,
    pub starter_set: CharStarterSet,
    pub gender: Gender,
}
packet_opcode!(CreateCharReq, RecvOpcodes::CreateNewCharacter);

#[derive(MooplePacket, Debug)]
pub struct CreateCharSale {
    pub name: String,
    pub job: JobGroup,
    pub sale_job: u32,
    pub starter_set: CharStarterSet,
}
packet_opcode!(CreateCharSale, RecvOpcodes::CreateNewCharacterInCS);

#[derive(MooplePacket, Debug)]
pub struct CheckDuplicateIDReq {
    pub name: String,
}
packet_opcode!(CheckDuplicateIDReq, RecvOpcodes::CheckDuplicatedID);

#[derive(Debug, Clone, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum CheckDuplicateIDResult {
    Success = 0,
    // TODO: mapped to 5
    Error1 = 1,
    // map to 10
    Error2 = 2,
    // map to 18 or well every code aside from 0,1,2
    Error3 = 3,
}
mark_maple_enum!(CheckDuplicateIDResult);

#[derive(MooplePacket, Debug)]
pub struct CheckDuplicateIDResp {
    pub name: String,
    pub result: CheckDuplicateIDResult,
}
packet_opcode!(CheckDuplicateIDResp, SendOpcodes::CheckDuplicatedIDResult);
