use std::{collections::BTreeMap, sync::Arc};

use proto95::id::SkillId;

use crate::{
    entities::{self, skill},
    services::{
        character::Character,
        data::{character::CharacterID, DataServices},
        helper::intentory::inv::InventorySet,
    },
};

use super::session_manager::{OwnedSession, SessionBackend};

#[derive(Debug, Clone)]
pub struct MoopleSessionData {
    pub acc: entities::account::Model,
    pub char: Character,
    pub inv: InventorySet,
    pub skills: BTreeMap<SkillId, skill::Model>,
}

pub type OwnedMoopleSession = OwnedSession<uuid::Uuid, MoopleSessionData>;

#[derive(Debug)]
pub struct MoopleSessionBackend {
    pub(crate) data: Arc<DataServices>,
}

#[async_trait::async_trait]
impl SessionBackend for MoopleSessionBackend {
    type SessionData = MoopleSessionData;
    type SessionLoadParam = (entities::account::Model, CharacterID);

    async fn load(&self, param: Self::SessionLoadParam) -> anyhow::Result<Self::SessionData> {
        let (acc, char_id) = param;
        //TODO: important verify that char belongs to the account
        let char = Character::from(self.data.char.must_get(char_id).await?);
        let inv = self.data.item.load_inventory_for_character(char_id).await?;

        let skills = self
            .data
            .char
            .load_skills(char_id)
            .await?
            .into_iter()
            .map(|skill| (SkillId(skill.id as u32), skill))
            .collect();
        Ok(MoopleSessionData {
            acc,
            char,
            inv,
            skills,
        })
    }
    async fn save(&self, session: Self::SessionData) -> anyhow::Result<()> {
        let char_id = session.char.model.id;
        self.data.item.save_inventory(session.inv, char_id).await?;

        Ok(())
    }
}
