use std::collections::BTreeMap;

use data::entities::skill;
use proto95::{shared::char::CharStatPartial, stats::PartialFlag, id::SkillId};

pub type PartialCharStats = PartialFlag<(), CharStatPartial>;

#[derive(Debug)]
pub struct CharState {
    _skills: BTreeMap<SkillId, skill::Model>,
    _char_data: PartialCharStats
}