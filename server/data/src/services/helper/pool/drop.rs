use std::{ops::Add, time::Duration};

use moople_packet::proto::time::MapleExpiration;
use proto95::{
    game::{
        drop::{
            DropEnterFieldResp, DropEnterType, DropLeaveFieldResp, DropLeaveType, DropOwner,
            DropType,
        },
        mob::MobId,
        ObjectId,
    },
    id::ItemId,
    shared::{char::CharacterId, Vec2},
};

use crate::services::{session::session_set::SessionSet, data::character::CharacterID};

use super::{Pool, PoolItem, next_id};

#[derive(Debug)]
pub struct Drop {
    pub owner: DropOwner,
    pub pos: Vec2,
    pub start_pos: Vec2,
    pub value: DropTypeValue,
    pub quantity: usize
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


    fn get_id(&self) -> Self::Id {
        next_id()
    }

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket {
        let (drop_type, expiration) = match self.value {
            DropTypeValue::Item(item) => (
                DropType::Item(item),
                Some(MapleExpiration::delay(chrono::Duration::seconds(60))),
            ),
            DropTypeValue::Mesos(mesos) => (DropType::Money(mesos), None),
        };

        let start_pos = (
            self.start_pos.add((0, -100).into()),
            Duration::from_millis(1000).into(),
        );

        DropEnterFieldResp {
            enter_type: DropEnterType::Create,
            id,
            drop_type,
            drop_owner: self.owner.clone(),
            pos: self.pos,
            src_id: 0,
            start_pos: Some(start_pos).into(),
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

impl Pool<Drop> {
    pub async fn add_mob_drops(
        &self,
        killed_mob: MobId,
        pos: Vec2,
        killer: CharacterID,
        sessions: &SessionSet,
    ) -> anyhow::Result<()> {
        let Some(drops) = self.meta.get_drops_for_mob(killed_mob)  else {
            return Ok(())
        };

        let money = drops.get_money_drop(&mut rand::thread_rng());
        let items = drops.get_item_drops(&mut rand::thread_rng());
        if money > 0 {
            self.add(
                Drop {
                    owner: DropOwner::User(killer as u32),
                    pos,
                    start_pos: pos,
                    value: DropTypeValue::Mesos(money),
                    quantity: 1
                },
                sessions,
            )
            .await?;
        }

        for (item, quantity) in items {
            self.add(
                Drop {
                    owner: DropOwner::User(killer as u32),
                    pos,
                    start_pos: pos,
                    value: DropTypeValue::Item(item),
                    quantity
                },
                sessions,
            )
            .await?;
        }

        Ok(())
    }
}
