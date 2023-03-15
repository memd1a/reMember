use moople_derive::MooplePacket;
use moople_packet::{
    maple_enum_code, packet_opcode,
    proto::{time::{MapleDurationMs16, MapleTime}, CondOption}, maple_packet_enum,
};

use crate::{
    id::ItemId,
    send_opcodes::SendOpcodes,
    shared::{char::CharacterId, Vec2},
};

pub type DropId = u32;

maple_enum_code!(
    DropOwnerType,
    u8,
    UserOwner = 0,
    PartyOwner = 1,
    NoOwner = 2,
    ExplosiveNoOwner = 3
);

maple_enum_code!(
    DropEnterType,
    u8,
    None = 0,
    Create = 1,
    OnFoothold = 2,
    FadingOut = 3,
    Unknown4 = 4
);

impl DropEnterType {
    fn has_start_pos(&self) -> bool {
        matches!(self, Self::None | Self::Create | Self::FadingOut | Self::Unknown4)
    }
}

maple_packet_enum!(
    DropType,
    u8,
    Item(ItemId) => 0,
    Money(u32) => 1
);

impl DropType {
    fn has_expiration(&self) -> bool {
        !matches!(self, DropType::Money(_))
    }
}

#[derive(MooplePacket, Debug)]
pub struct DropEnterFieldResp {
    pub enter_type: DropEnterType,
    pub id: DropId,
    pub drop_type: DropType,
    pub owner_id: CharacterId,
    pub drop_owner_type: DropOwnerType,
    pub pos: Vec2,
    pub src_id: u32,
    #[pkt(if(field="enter_type", cond="DropEnterType::has_start_pos"))]
    pub start_pos: CondOption<(Vec2, MapleDurationMs16)>,
    #[pkt(if(field="drop_type", cond="DropType::has_expiration"))]
    pub drop_expiration: CondOption<MapleTime>,
    //TODO: ? ownerCharId == 0
    pub by_pet: bool,
    // If this is set to true It throws an exception
    pub u1_flag: bool,
}
packet_opcode!(DropEnterFieldResp, SendOpcodes::DropEnterField);


maple_enum_code!(
    DropLeaveType,
    u8,
    TimeOut = 0,
    ScreenScroll = 1,
    UserPickup = 2,
    MobPickup = 3,
    Explode = 4,
    PetPickup = 5,
    PassConvex = 6,
    PetSkill = 7
);

impl DropLeaveType {
    fn has_pickup_id(&self) -> bool {
        matches!(self, Self::UserPickup | Self::MobPickup | Self::PetSkill)
    }
}

#[derive(MooplePacket, Debug)]
pub struct DropLeaveFieldResp {
    pub leave_type: DropLeaveType,
    pub id: DropId,
    #[pkt(if( field="leave_type", cond="DropLeaveType::has_pickup_id"))]
    pub pickup_id: CondOption<u32>

}
packet_opcode!(DropLeaveFieldResp, SendOpcodes::DropLeaveField);