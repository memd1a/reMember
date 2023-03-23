use sea_orm_migration::prelude::*;

use super::char_stat;

pub const CHAR_STATS: [&str; 25] = [
    "level",
    "exp",
    "gacha_exp",
    "str",
    "dex",
    "luk",
    "int",
    "hp",
    "max_hp",
    "mp",
    "max_mp",
    "mesos",
    "map_id",
    "buddy_capacity",
    "fame",
    "sp",
    "ap",
    "job",
    "equip_slots",
    "use_slots",
    "setup_slots",
    "etc_slots",
    "face",
    "skin",
    "hair",
];

pub fn with_char_stats(columns: impl IntoIterator<Item = ColumnDef>) -> Vec<ColumnDef> {
    columns
        .into_iter()
        .chain(CHAR_STATS.iter().map(|stat| char_stat(Alias::new(stat))))
        .collect()
}

pub const ITEM_STATS: [&str; 17] = [
    "level",
    "upgrade_slots",
    "str",
    "dex",
    "luk",
    "int",
    "hp",
    "mp",
    "weapon_atk",
    "magic_atk",
    "weapon_def",
    "magic_def",
    "accuracy",
    "avoid",
    "craft",
    "speed",
    "jump",
];


pub fn with_equip_stats(columns: impl IntoIterator<Item = ColumnDef>) -> Vec<ColumnDef> {
    columns
        .into_iter()
        .chain(ITEM_STATS.iter().map(|stat| char_stat(Alias::new(stat))))
        .collect()
}
