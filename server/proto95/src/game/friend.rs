use moople_derive::MooplePacket;
use moople_packet::{
    maple_packet_enum, packet_opcode, proto::{option::MapleOption8, string::FixedPacketString}, DecodePacket, EncodePacket,
    PacketLen,
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

#[derive(Debug)]
pub struct FriendList {
    pub friends: Vec<FriendRecord>,
    pub in_shop: Vec<u32>, // 4 bit boolean
}

impl FriendList {
    pub fn empty() -> Self {
        Self {
            friends: Vec::new(),
            in_shop: Vec::new(),
        }
    }
}

//TODO solve this via derive or atleast auto-impl Encode + PacketLen
impl<'de> DecodePacket<'de> for FriendList {
    fn decode_packet(
        pr: &mut moople_packet::MaplePacketReader<'de>,
    ) -> moople_packet::NetResult<Self> {
        let n = pr.read_u8()? as usize;

        Ok(Self {
            friends: FriendRecord::decode_packet_n(pr, n)?,
            in_shop: u32::decode_packet_n(pr, n)?,
        })
    }
}

impl EncodePacket for FriendList {
    fn encode_packet<T: bytes::BufMut>(
        &self,
        pw: &mut moople_packet::MaplePacketWriter<T>,
    ) -> moople_packet::NetResult<()> {
        let n = self.friends.len();
        assert_eq!(self.in_shop.len(), n);

        pw.write_u8(n as u8);
        self.friends.encode_packet(pw)?;
        self.in_shop.encode_packet(pw)?;

        Ok(())
    }
}

impl PacketLen for FriendList {
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        1 // size
        + self.friends.packet_len()
        + self.in_shop.packet_len()
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
