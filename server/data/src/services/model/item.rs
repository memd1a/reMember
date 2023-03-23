use std::ops::{Deref, DerefMut, RangeInclusive};

use chrono::NaiveDateTime;
use enum_map::{enum_map, Enum, EnumMap};
use moople_packet::proto::time::{MapleExpiration, MapleTime};
use proto95::{
    id::ItemId,
    shared::item::{self as proto_item},
};
use rand::Rng;

use crate::{
    entities::{equip_item, item_stack},
    services::meta::meta_service::{get_equip_stats, ItemMeta},
};

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
    Craft,
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
    pub stats: EquipStats,
    pub upgrades: u8,
    pub slots: u8,
    pub hammers_used: u8,
    pub level_info: Option<ItemLevelInfo>,
}

impl From<equip_item::Model> for EquipItem {
    fn from(value: equip_item::Model) -> Self {
        let owner = if value.owner_tag.is_empty() {
            None
        } else {
            Some(value.owner_tag.clone())
        };

        //TODO
        let stats = enum_map! {
            EquipStat::Str => value.str as u16,
            EquipStat::Dex => value.dex as u16,
            EquipStat::Luk => value.luk as u16,
            EquipStat::Int => value.int as u16,
            EquipStat::Hp => value.hp as u16,
            EquipStat::Mp => value.mp as u16,
            EquipStat::WeaponAtk => value.weapon_atk as u16,
            EquipStat::WeaponDef => value.weapon_def as u16,
            EquipStat::MagicAtk => value.magic_atk as u16,
            EquipStat::MagicDef => value.magic_def as u16,
            EquipStat::Accuracy => value.accuracy as u16,
            EquipStat::Avoid => value.avoid as u16,
            EquipStat::Speed => value.speed as u16,
            EquipStat::Jump => value.jump as u16,
            EquipStat::Craft => value.craft as u16,
        };
        Self {
            info: ItemInfo {
                db_id: Some(value.id),
                item_id: ItemId(value.item_id as u32),
                cash_id: value.cash_id.map(|i| i as u64),
                expiration: value.expires_at,
                owner: owner,
                flags: proto_item::ItemFlags::from_bits(value.flags as u16).unwrap(),
                last_update: 0,
            },
            hammers_used: value.vicious_hammers as u8,
            level_info: Some(ItemLevelInfo {
                level: value.level as u8, //TODO
                exp: value.item_exp as u32,
            }),
            slots: value.upgrade_slots as u8,
            upgrades: 0,
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

fn rnd_stat(mut rng: impl Rng, stat: u16) -> u16 {
    if stat == 0 {
        return 0;
    }

    rng.gen_range(stat.wrapping_sub(2)..=stat).max(1)
}

impl EquipItem {
    pub fn from_item_id(item_id: ItemId, meta: ItemMeta) -> Self {
        let mut rng = rand::thread_rng();
        let stats = get_equip_stats(meta).map(|_, v| rnd_stat(&mut rng, v));
        Self {
            info: ItemInfo::from_id(item_id),
            stats,
            slots: meta.slot_max as u8,
            hammers_used: 0,
            level_info: None,
            upgrades: 0,
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

impl From<&EquipItem> for proto_item::EquipItemInfo {
    fn from(value: &EquipItem) -> Self {
        proto_item::EquipItemInfo {
            info: proto_item::ItemInfo {
                item_id: value.item_id,
                cash_id: value.cash_id.into(),
                expiration: value.expiration.into(),
            },
            stats: proto_item::EquipAllStats {
                remaining_upgrade_slots: value.slots,
                upgrade_count: value.upgrades,
                stats: map_eq_stats(&value.stats),
                title: value.owner.clone().unwrap_or("".to_string()),
                flags: value.flags,
            },
            time_stamp: MapleTime::permanent(),
            lvl_up_ty: 0,
            lvl: value.level_info.as_ref().map(|l| l.level).unwrap_or(0),
            exp: value.level_info.as_ref().map(|l| l.exp).unwrap_or(0),
            durability: -1,
            hammer_count: value.hammers_used as u32,
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
        craft: stats[EquipStat::Craft],
        speed: stats[EquipStat::Speed],
        jump: stats[EquipStat::Jump],
    }
}

impl From<&StackItem> for proto_item::ItemStackData {
    fn from(value: &StackItem) -> Self {
        proto_item::ItemStackData {
            info: proto_item::ItemInfo {
                item_id: value.item_id,
                cash_id: value.cash_id.into(),
                expiration: MapleExpiration::never(),
            },
            quantity: value.quantity,
            title: value.owner.clone().unwrap_or("aaa".to_string()),
            flag: value.flags,
            serial_number: None.into(),
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
