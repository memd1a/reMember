use moople_derive::MooplePacket;
use moople_packet::{
    maple_packet_enum, packet_opcode,
    proto::{option::MapleOption8, string::FixedPacketString},
};

use crate::{
    send_opcodes::SendOpcodes,
    shared::{char::CharacterId, NameStr},
};

//TODO in_shop is an u8 idk

#[derive(MooplePacket, Debug)]
pub struct FriendRecord {
    pub id: CharacterId,
    pub name: NameStr,
    pub flag: u8,
    pub channel_id: u32,
    pub friend_group: FixedPacketString<0x11>,
}

#[derive(MooplePacket, Debug)]
pub struct FriendList {
    pub len: u8,
    #[pkt(size = "len")]
    pub friends: Vec<FriendRecord>,
    #[pkt(size = "len")]
    pub in_shop: Vec<u32>, // 4 bit boolean
}

impl FriendList {
    pub fn empty() -> Self {
        Self {
            len: 0,
            friends: Vec::new(),
            in_shop: Vec::new(),
        }
    }
}

#[derive(MooplePacket, Debug)]
pub struct FriendUpdate {
    pub friend_id: CharacterId,
    pub record: FriendRecord,
    pub in_shop: bool,
}

#[derive(MooplePacket, Debug)]
pub struct FriendChangeChannel {
    pub friend_id: CharacterId,
    pub in_shop: bool,
    pub channel: u32,
}

#[derive(MooplePacket, Debug)]
pub struct FriendUnknown9 {
    pub friend_id: CharacterId,
    pub in_shop: bool,
    pub channel_id: u32,
}

#[derive(MooplePacket, Debug)]
pub struct FriendReq {
    pub friend_id: CharacterId,
    pub friend_name: String,
    pub level: u32,
    pub job_code: u32, //TODO: job id?
}

maple_packet_enum!(
    FriendResultResp,
    u8,
    Reset(FriendList) => 0,
    Update(FriendUpdate) => 1,
    Req(FriendReq) => 2,
    Reset3(FriendList) => 3,
    Unknown4(()) => 4,
    Unknown5(()) => 5,
    Unknown6(()) => 6,
    Unknown7(()) => 7,
    Unknown8(()) => 8,
    Unknown9(MapleOption8<String>) => 9,
    UnknownA(MapleOption8<String>) => 0xa,
    // Blocked is alwayws true for this
    ResetB(FriendList) => 0xb,
    UnknownC(MapleOption8<String>) => 0xc,
    ChangeChannel(FriendChangeChannel) => 0xd,
    MaxFriends(u8) => 0xe,
    UnknownF(MapleOption8<String>) => 0xf,
);
packet_opcode!(FriendResultResp, SendOpcodes::FriendResult);
