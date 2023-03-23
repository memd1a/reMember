use moople_derive::MooplePacket;
use moople_packet::{
    maple_enum_code, maple_packet_enum, packet_opcode,
    proto::{
        time::{MapleDurationMs16, MapleExpiration},
        CondOption, MapleTryWrapped,
    },
    NetError,
};

use crate::{
    id::ItemId,
    send_opcodes::SendOpcodes,
    shared::{char::CharacterId, Vec2},
};

pub type DropId = u32;

#[derive(Debug, Clone)]
pub enum DropOwner {
    User(CharacterId),
    // TODO: Party ID
    Party(u32),
    None,
    Explosive,
}

impl MapleTryWrapped for DropOwner {
    type Inner = (u32, u8);

    fn maple_into_inner(&self) -> Self::Inner {
        match self {
            DropOwner::User(user) => (*user, 0),
            DropOwner::Party(party) => (*party, 1),
            DropOwner::None => (0, 2),
            DropOwner::Explosive => (0, 3),
        }
    }

    fn maple_try_from(v: Self::Inner) -> moople_packet::NetResult<Self> {
        Ok(match v.1 {
            0 => Self::User(v.0),
            1 => Self::Party(v.0),
            2 => Self::None,
            3 => Self::Explosive,
            _ => return Err(NetError::InvalidEnumPrimitive(v.1 as u32)),
        })
    }
}

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
        matches!(
            self,
            Self::None | Self::Create | Self::FadingOut | Self::Unknown4
        )
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
    pub drop_owner: DropOwner,
    pub pos: Vec2,
    pub src_id: u32,
    #[pkt(if(field = "enter_type", cond = "DropEnterType::has_start_pos"))]
    pub start_pos: CondOption<(Vec2, MapleDurationMs16)>,
    #[pkt(if(field = "drop_type", cond = "DropType::has_expiration"))]
    pub drop_expiration: CondOption<MapleExpiration>,
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
    #[pkt(if(field = "leave_type", cond = "DropLeaveType::has_pickup_id"))]
    pub pickup_id: CondOption<u32>,
}
packet_opcode!(DropLeaveFieldResp, SendOpcodes::DropLeaveField);
