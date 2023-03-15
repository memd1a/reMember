use crate::{entities, services::helper::intentory::inv::InventorySet};

use super::session_manager::SessionDataHolder;

#[derive(Debug, Clone)]
pub struct MoopleSessionData {
    pub acc: entities::account::Model,
    pub char: entities::character::Model,
    pub inv: InventorySet,
}



pub type MoopleSessionHolder = SessionDataHolder<MoopleSessionData>;