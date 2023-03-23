use std::collections::BTreeMap;

use proto95::{id::SkillId, login::ClientKey};

use crate::{entities::{self, skill}, services::helper::intentory::inv::InventorySet};

use super::session_manager::SessionDataHolder;

#[derive(Debug, Clone)]
pub struct MoopleSessionData {
    pub acc: entities::account::Model,
    pub char: entities::character::Model,
    pub inv: InventorySet,
    pub skills: BTreeMap<SkillId, skill::Model>,
}


pub type MoopleSessionHolder = SessionDataHolder<MoopleSessionData>;