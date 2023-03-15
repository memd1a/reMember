use std::ops::Add;

use moople_packet::proto::time::MapleTime;
use proto95::{
    game::{
        drop::{
            DropEnterFieldResp, DropEnterType, DropLeaveFieldResp, DropLeaveType, DropOwnerType,
            DropType,
        },
        ObjectId,
    },
    id::ItemId,
    shared::{char::CharacterId, Vec2},
};

use super::PoolItem;

#[derive(Debug)]
pub struct Drop {
    pub owner: CharacterId,
    pub pos: Vec2,
    pub start_pos: Vec2,
    pub value: DropTypeValue,
}

#[derive(Debug)]
pub enum DropTypeValue {
    Mesos(u32),
    Item(ItemId),
}

#[derive(Debug)]
pub enum DropLeaveParam {
    TimeOut,
    ScreenScroll,
    UserPickup(u32),
    MobPickup(u32),
    Explode,
    PetPickup(u32),
    PassConvex,
    PetSkill,
}

impl PoolItem for Drop {
    type Id = ObjectId;

    type EnterPacket = DropEnterFieldResp;
    type LeavePacket = DropLeaveFieldResp;
    type LeaveParam = DropLeaveParam;

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket {
        dbg!(self);
        let (drop_type, expiration) = match self.value {
            DropTypeValue::Item(item) => {
                let expiration = chrono::Utc::now()
                    .naive_utc()
                    .add(chrono::Duration::seconds(60));
                (DropType::Item(item), Some(MapleTime::from(expiration)))
            }
            DropTypeValue::Mesos(mesos) => (DropType::Money(mesos), None),
        };

        DropEnterFieldResp {
            enter_type: DropEnterType::OnFoothold,
            id,
            drop_type,
            owner_id: self.owner,
            drop_owner_type: DropOwnerType::UserOwner,
            pos: self.pos,
            src_id: 0,
            start_pos: None.into(),
            drop_expiration: expiration.into(),
            by_pet: false,
            u1_flag: false,
        }
    }

    fn get_leave_pkt(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeavePacket {
        let (leave_type, pickup_id) = match param {
            DropLeaveParam::Explode => (DropLeaveType::Explode, None),
            DropLeaveParam::PassConvex => (DropLeaveType::PassConvex, None),
            DropLeaveParam::PetSkill => (DropLeaveType::PetSkill, None),
            DropLeaveParam::ScreenScroll => (DropLeaveType::ScreenScroll, None),
            DropLeaveParam::TimeOut => (DropLeaveType::TimeOut, None),
            DropLeaveParam::UserPickup(id) => (DropLeaveType::UserPickup, Some(id)),
            DropLeaveParam::MobPickup(id) => (DropLeaveType::MobPickup, Some(id)),
            DropLeaveParam::PetPickup(id) => (DropLeaveType::PetPickup, Some(id)),
        };

        DropLeaveFieldResp {
            leave_type,
            id,
            pickup_id: pickup_id.into(),
        }
    }
}
