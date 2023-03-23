use std::collections::BTreeMap;

use data::entities::skill;
use proto95::{shared::char::CharStatPartial, stats::PartialFlag, id::SkillId};

pub type PartialCharStats = PartialFlag<(), CharStatPartial>;

#[derive(Debug)]
pub struct CharState {
    skills: BTreeMap<SkillId, skill::Model>,
    char_data: PartialCharStats
}