use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
};

use game_data::{map, wz2};
use proto95::{
    game::mob::MobId,
    id::{ItemId, MapId},
};
use rand::Rng;

use crate::services::model::item::{EquipStats, EquipStat};

#[derive(Debug)]
pub struct DropEntry {
    pub item: ItemId,
    pub max_quantity: usize,
    pub chance: f32,
}

#[derive(Debug)]
pub struct DropPool {
    pub entries: Vec<DropEntry>,
    pub money: u32,
    pub money_variance: u32,
}

impl DropPool {
    pub fn get_item_drops<R: Rng>(&self, rng: &mut R) -> Vec<(ItemId, usize)> {
        let mut drops = Vec::new();
        for entry in self.entries.iter() {
            if !rng.gen_bool(entry.chance.into()) {
                continue;
            }

            drops.push((entry.item.clone(), rng.gen_range(1..=entry.max_quantity)))
        }
        drops
    }

    pub fn get_money_drop<R: Rng>(&self, rng: &mut R) -> u32 {
        rng.gen_range((self.money - self.money_variance)..=self.money)
    }
}

pub fn get_equip_stats(meta: ItemMeta) -> EquipStats {
    enum_map::enum_map! {
        EquipStat::Str => meta.inc_str as u16,
        EquipStat::Dex => meta.inc_dex as u16,
        EquipStat::Int => meta.inc_int as u16,
        EquipStat::Luk => meta.inc_luk as u16,
        EquipStat::Hp => meta.inc_max_hp as u16,
        EquipStat::Mp => meta.inc_max_mp as u16,
        EquipStat::WeaponAtk => meta.inc_pad as u16,
        EquipStat::MagicAtk => meta.inc_mad as u16,
        EquipStat::WeaponDef => meta.inc_pdd as u16,
        EquipStat::MagicDef => meta.inc_mdd as u16,
        EquipStat::Accuracy => meta.inc_acc as u16,
        EquipStat::Avoid => meta.inc_eva as u16,
        EquipStat::Craft => meta.inc_craft as u16,
        EquipStat::Speed => meta.inc_speed as u16,
        EquipStat::Jump => meta.inc_jump as u16
    }
}

#[derive(Debug)]
pub struct MetaData {
    pub maps0: BTreeMap<i64, map::Map>,
    pub mobs: BTreeMap<u32, wz2::Mob>,
    pub items: BTreeMap<u32, wz2::Item>,
}

pub type FieldMeta = &'static map::Map;
pub type MobMeta = &'static wz2::Mob;
pub type ItemMeta = &'static wz2::Item;
pub type DropsMeta = &'static DropPool;

impl MetaData {
    fn load_from_file<T: serde::de::DeserializeOwned>(file: impl AsRef<Path>) -> anyhow::Result<T> {
        let file = File::open(file)?;
        Ok(bincode::deserialize_from(file)?)
    }

    pub fn load_from_dir(dir: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            maps0: Self::load_from_file(dir.join("maps0.rbin"))?,
            mobs: wz2::load_all(dir.join("wz/Mob"))?,
            items: wz2::load_all(dir.join("wz/Item"))?,
        })
    }
}

#[derive(Debug)]
pub struct MetaService {
    meta_data: MetaData,
    hard_coded_drop_pool: DropPool
}

impl MetaService {
    pub fn new(meta_data: MetaData) -> Self {
        let hard_coded_drop_pool = DropPool {
            entries: vec![
                DropEntry {
                    item: ItemId::ADVANCED_MONSTER_CRYSTAL_1,
                    max_quantity: 5,
                    chance: 0.5,
                },
                DropEntry {
                    item: ItemId::PINK_ADVENTURER_CAPE,
                    max_quantity: 1,
                    chance: 0.7,
                },
            ],
            money: 1_000,
            money_variance: 970,
        };
        Self { meta_data, hard_coded_drop_pool }
    }

    pub fn load_from_dir(dir: PathBuf) -> anyhow::Result<Self> {
        Ok(Self::new(MetaData::load_from_dir(dir)?))
    }

    pub fn get_field_data(&'static self, field_id: MapId) -> Option<FieldMeta> {
        self.meta_data.maps0.get(&(field_id.0 as i64))
    }

    pub fn get_mob_data(&'static self, mob_id: MobId) -> Option<MobMeta> {
        self.meta_data.mobs.get(&mob_id)
    }

    pub fn get_item_data(&'static self, id: ItemId) -> Option<ItemMeta> {
        self.meta_data.items.get(&id.0)
    }

    pub fn get_drops_for_mob(&'static self, _id: MobId) -> Option<DropsMeta> {
        Some(&self.hard_coded_drop_pool)
    }
}
