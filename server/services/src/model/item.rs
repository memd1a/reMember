use std::ops::{Deref, DerefMut, RangeInclusive};

use chrono::NaiveDateTime;
use data::entities::{equip_item, item_stack};
use enum_map::{enum_map, Enum, EnumMap};
use proto95::{id::ItemId, shared::item::ItemFlags};



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
    pub flags: ItemFlags,
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
            flags: ItemFlags::empty(),
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
                flags: ItemFlags::from_bits(value.flags as u16).unwrap(), //TODO ::from(value.flags as u16),
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
                flags: ItemFlags::from_bits(value.flags as u16).unwrap(), //TODO ::from(value.flags as u16),
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
