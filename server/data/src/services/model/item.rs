use std::ops::{Deref, DerefMut, RangeInclusive};

use chrono::NaiveDateTime;
use enum_map::{enum_map, Enum, EnumMap};
use moople_packet::proto::time::MapleTime;
use proto95::{
    id::ItemId,
    shared::item::{self as proto_item},
};

use crate::entities::{equip_item, item_stack};

#[derive(Debug, Enum, Clone)]
pub enum EquipStat {
    Str,
    Dex,
    Int,
    Luk,
    Hp,
    Mp,
    WeaponAtk,
    MagicAtk,
    WeaponDef,
    MagicDef,
    Accuracy,
    Avoid,
    Hands,
    Speed,
    Jump,
}

pub type EquipStats = EnumMap<EquipStat, u16>;

#[derive(Debug, Clone)]
pub struct ItemLevelInfo {
    pub level: u8,
    pub exp: u32,
}

#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub db_id: Option<i32>,
    pub item_id: ItemId,
    pub cash_id: Option<u64>,
    pub expiration: Option<NaiveDateTime>,
    pub owner: Option<String>,
    pub flags: proto_item::ItemFlags,
    pub last_update: u32,
}

impl ItemInfo {
    pub fn from_id(item_id: ItemId) -> Self {
        Self {
            db_id: None,
            item_id,
            cash_id: None,
            expiration: None,
            owner: None,
            flags: proto_item::ItemFlags::empty(),
            last_update: 0,
        }
    }

    pub fn is_expired(&self, now: NaiveDateTime) -> bool {
        match self.expiration {
            Some(t_exp) => t_exp <= now,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]

pub struct EquipItem {
    pub info: ItemInfo,
    pub level: u8,
    pub stats: EquipStats,
    pub slots: u8,
    pub hammers_used: u8,
    pub level_info: Option<ItemLevelInfo>,
}

impl From<equip_item::Model> for EquipItem {
    fn from(value: equip_item::Model) -> Self {
        //TODO
        let stats = enum_map! {
            EquipStat::Str => value.str as u16,
            EquipStat::WeaponAtk => value.weapon_atk as u16,
            _ => 0
        };
        Self {
            info: ItemInfo {
                db_id: Some(value.id),
                item_id: ItemId(value.item_id as u32),
                cash_id: value.cash_id.map(|i| i as u64),
                expiration: value.expires_at,
                owner: None,
                flags: proto_item::ItemFlags::from_bits(value.flags as u16).unwrap(), //TODO ::from(value.flags as u16),
                last_update: 0,
            },
            level: value.level as u8,
            hammers_used: value.vicious_hammers as u8,
            level_info: Some(ItemLevelInfo {
                level: 0, //TODO
                exp: value.item_exp as u32,
            }),
            slots: value.upgrade_slots as u8,
            stats,
        }
    }
}

impl Deref for EquipItem {
    type Target = ItemInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl DerefMut for EquipItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}

impl EquipItem {
    pub fn from_item_id(item_id: ItemId) -> Self {
        Self {
            info: ItemInfo::from_id(item_id),
            level: 0,
            stats: EquipStats::default(),
            slots: 0,
            hammers_used: 0,
            level_info: None,
        }
    }

    pub fn apply_chaos_scroll(
        &mut self,
        mut rng: impl rand::Rng,
        chance: f64,
        range: RangeInclusive<i16>,
    ) -> bool {
        let success = rng.gen_bool(chance);
        if !success {
            return false;
        }

        for val in self.stats.values_mut() {
            if *val == 0 {
                continue;
            }

            let stat_diff = rng.gen_range(range.clone());
            *val = val.saturating_add_signed(stat_diff);
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct StackItem {
    pub info: ItemInfo,
    pub quantity: u16,
}

impl Deref for StackItem {
    type Target = ItemInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl DerefMut for StackItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}

impl From<item_stack::Model> for StackItem {
    fn from(value: item_stack::Model) -> Self {
        Self {
            info: ItemInfo {
                db_id: Some(value.id),
                item_id: ItemId(value.item_id as u32),
                cash_id: value.cash_id.map(|i| i as u64),
                expiration: value.expires_at,
                owner: None,
                flags: proto_item::ItemFlags::from_bits(value.flags as u16).unwrap(), //TODO ::from(value.flags as u16),
                last_update: 0,
            },
            quantity: value.quantity as u16,
        }
    }
}

impl StackItem {
    pub fn from_item_id(item_id: ItemId, quantity: u16) -> Self {
        Self {
            info: ItemInfo::from_id(item_id),
            quantity,
        }
    }
}

fn map_expiration(dt: Option<NaiveDateTime>) ->  MapleTime {
    dt.map(MapleTime::from).unwrap_or(MapleTime(0))
}

 /* 
impl From<&equip_item::Model> for proto_item::EquipItemInfo {
    fn from(value: &equip_item::Model) -> Self {


        Self {
            info: proto_item::ItemInfo {
                item_id: ItemId(value.item_id as u32),
                cash_id: value.cash_id.map(|v| v as u64).into(),
                expiration: map_expiration(value.expires_at)
            },
            stats: todo!(),
            lvl_up_ty: todo!(),
            lvl: todo!(),
            exp: todo!(),
            durability: todo!(),
            hammer_count: todo!(),
            grade: todo!(),
            stars: todo!(),
            options: todo!(),
            sockets: todo!(),
            sn: value.id as u64,
            time_stamp: MapleTime::permanent(),
            prev_bonus_exp_rate: -1,
        }
    }
}*/

impl From<&EquipItem> for proto_item::EquipItemInfo {
    fn from(value: &EquipItem) -> Self {
        proto_item::EquipItemInfo {
            info: proto_item::ItemInfo {
                item_id: value.item_id,
                cash_id: value.cash_id.into(),
                expiration: map_expiration(value.expiration)
            },
            stats: proto_item::EquipAllStats {
                remaining_upgrade_slots: value.slots,
                upgrade_count: 1,
                stats: map_eq_stats(&value.stats),
                title: value.owner.clone().unwrap_or("aaa".to_string()),
                flags: value.flags,
            },
            time_stamp: MapleTime::permanent(),
            lvl_up_ty: 0,
            lvl: 0,
            exp: 0,
            durability: -1,
            hammer_count: 0,
            grade: 0,
            stars: 3,
            options: [0; 3],
            sockets: [0; 2],
            sn: value.db_id.unwrap() as u64,
            prev_bonus_exp_rate: -1,
        }
    }
}

fn map_eq_stats(stats: &EquipStats) -> proto95::shared::item::EquipStats {
    proto95::shared::item::EquipStats {
        str: stats[EquipStat::Str],
        dex: stats[EquipStat::Dex],
        int: stats[EquipStat::Int],
        luk: stats[EquipStat::Luk],
        hp: stats[EquipStat::Hp],
        mp: stats[EquipStat::Mp],
        watk: stats[EquipStat::WeaponAtk],
        matk: stats[EquipStat::MagicAtk],
        wdef: stats[EquipStat::WeaponDef],
        mdef: stats[EquipStat::MagicDef],
        accuracy: stats[EquipStat::Accuracy],
        avoid: stats[EquipStat::Avoid],
        craft: /*stats[EquipStat::Craft] as u16,*/ 0,//TODO !!
        speed: stats[EquipStat::Speed] + 100,
        jump: stats[EquipStat::Jump] + 100,
    }
}



impl From<&StackItem> for proto_item::ItemStackData {
    fn from(value: &StackItem) -> Self {
        proto_item::ItemStackData {
            info: proto_item::ItemInfo {
                item_id: value.item_id,
                cash_id: value.cash_id.into(),
                expiration: MapleTime(0),
            },
            quantity: value.quantity,
            title: value.owner.clone().unwrap_or("aaa".to_string()),
            flag: value.flags,
            serial_number: None.into()
        }
    }
}

/*

fn map_item_info(info: &services::model::item::ItemInfo) -> ItemInfo {
    todo!()
}

fn map_item(item: &EquipItem) -> Item {
    Item::Equip(EquipItemInfo {
        info: map_item_info(&item.info),
        stats: enum_map! {
            array: todo!(),
        },
        level_info: OptionalLevelInfo(None),
        time_stamp: MapleTime::zero(),
        unknown1: 0,
    })
}*/
