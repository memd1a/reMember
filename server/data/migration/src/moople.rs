use sea_orm_migration::prelude::*;

use crate::helper::moople_ty::char_stat;

pub const CHAR_STATS: [&str; 26] = [
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
    "ap",
    "sp",
    "job",
    "equip_slots",
    "use_slots",
    "setup_slots",
    "etc_slots",
    "face",
    "skin",
    "hair",
    "spawn_point"
];

pub fn with_char_stats(columns: impl IntoIterator<Item = ColumnDef>) -> Vec<ColumnDef> {
    let mut cols: Vec<ColumnDef> = columns.into_iter().collect();
    cols.extend(CHAR_STATS.iter().map(|stat| char_stat(Alias::new(stat))));

    cols
}
