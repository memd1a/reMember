use moople_derive::MooplePacket;
use moople_packet::{
    maple_enum_code, maple_packet_enum, packet_opcode,
    proto::{time::Ticks, MapleList8},
};

use crate::{id::ItemId, recv_opcodes::RecvOpcodes, send_opcodes::SendOpcodes, shared::char::CharacterId};

#[derive(Debug, MooplePacket)]
pub struct GeneralChatPacket {
    message: String,
    show: bool,
}

#[derive(Debug, MooplePacket)]
pub struct SpouseChatPacket {
    message: String,
}

maple_enum_code!(
    MultiChatPacketType,
    u8,
    Buddy = 1,
    Party = 2,
    Guild = 3,
    Alliance = 4
);

#[derive(Debug, MooplePacket)]
pub struct MultiChatPacket {
    ty: MultiChatPacketType,
    recipients: MapleList8<u32>,
    message: String,
}

#[derive(Debug, MooplePacket)]
pub struct WispherData {
    name: String,
    message: String,
}

maple_packet_enum!(
    WispherMessageType,
    u8,
    Location(String) => 1,
    Whispher(WispherData) => 2,
    Request(String) => 0x04,
    Result(String) => 0x08,
    Receiver(String) => 0x10,
    Blocked(String) => 0x20,
    LocationFriend(String) => 0x40
);

#[derive(Debug, MooplePacket)]
pub struct ItemGainInfoData {
    path: String,
    unknown1: u32,
}

#[derive(Debug, MooplePacket)]
pub struct ItemGainItemData {
    mode2: u8,
    item_id: ItemId,
    quantity: u32,
}

maple_packet_enum!(
    SlashChatMsgType,
    u8,
    CmdStrF9(()) => 0x3A,
    //lvl
    CmdStr725(u8) => 0x1E,
    // some id?
    Create(u32) => 0,
    // /d
    CmdStr717(u8) => 1,
    // /exp
    CmdStr718(u32) => 2,

);

#[derive(Debug, MooplePacket)]
pub struct SlashChatMsg {
    msg: SlashChatMsgType,
}

#[derive(Debug, MooplePacket)]
pub struct ChatMsgReq {
    pub ticks: Ticks,
    pub msg: String,
    pub only_balloon: bool,
}
packet_opcode!(ChatMsgReq, RecvOpcodes::UserChat);

#[derive(Debug, MooplePacket)]
pub struct WhisperData {
    pub ticks: Ticks,
    pub target: String,
    pub msg: String,
}

#[derive(Debug, MooplePacket)]
pub struct WhisperFindData {
    pub ticks: Ticks,
    pub target: String,
}

maple_packet_enum!(
    WhiperMsgReq,
    u8,
    Unknown(WhisperData) => 0x86,
    Whisper(WhisperData) => 6,
    WhisperFind(WhisperFindData) => 5,
);
packet_opcode!(WhiperMsgReq, RecvOpcodes::Whisper);


#[derive(MooplePacket, Debug)]
pub struct UserChatMsgResp {
    pub char: CharacterId,
    pub is_admin: bool,
    pub msg: String,
    pub only_balloon: bool
}
packet_opcode!(UserChatMsgResp, SendOpcodes::UserChat);