pub mod character;
pub mod data;
pub mod field;
pub mod helper;
pub mod meta;
pub mod model;
pub mod server_info;
pub mod session;

use std::{sync::Arc, time::Duration};

use proto95::{
    id::{job_id::JobGroup, FaceId, HairId, Skin},
    shared::Gender,
};
use sea_orm::{DatabaseConnection, DbErr};
use server_info::{ServerInfo, ServerService};

use crate::entities::sea_orm_active_enums::GenderTy;

use self::{
    data::{
        account::{AccountId, Region},
        character::{CharacterCreateDTO, CharacterID, ItemStarterSet},
        DataServices,
    },
    field::FieldService,
    meta::meta_service::MetaService,
    session::{session_data::MoopleSessionBackend, GameSessionManager},
};

pub type SharedServices = Arc<Services>;

#[derive(Debug)]
pub struct Services {
    pub data: Arc<DataServices>,
    pub server_info: ServerService,
    pub session_manager: GameSessionManager<MoopleSessionBackend>,
    pub field: FieldService,
    pub meta: &'static MetaService,
}

impl Services {
    pub fn new(
        db: DatabaseConnection,
        servers: impl IntoIterator<Item = ServerInfo>,
        meta: &'static MetaService,
    ) -> Self {
        let data = Arc::new(DataServices::new(db, meta));

        let session_backend = MoopleSessionBackend { data: data.clone() };

        Self {
            data,
            session_manager: GameSessionManager::new(session_backend, Duration::from_secs(30)),
            server_info: ServerService::new(servers),
            field: FieldService::new(meta),
            meta,
        }
    }

    pub async fn seeded_in_memory(
        servers: impl IntoIterator<Item = ServerInfo>,
        meta: &'static MetaService,
    ) -> Result<Self, DbErr> {
        let db = crate::gen_sqlite(crate::SQL_OPT_MEMORY).await?;
        Ok(Self::new(db, servers, meta))
    }

    pub fn as_shared(self) -> SharedServices {
        Arc::new(self)
    }

    pub async fn seed_acc_char(&self) -> anyhow::Result<(AccountId, CharacterID)> {
        let acc_id = self
            .data
            .account
            .create(
                "admin",
                "test123",
                Region::Europe,
                true,
                Some(GenderTy::Female),
            )
            .await?;

        let job = JobGroup::Legend;
        let _char_id = self
            .data
            .char
            .create_character(
                acc_id,
                CharacterCreateDTO {
                    name: "Aran".to_string(),
                    job_group: JobGroup::Adventurer,
                    face: FaceId::LEISURE_LOOK_M,
                    skin: Skin::Normal,
                    hair: HairId::BLACK_TOBEN,
                    starter_set: ItemStarterSet {
                        bottom: job.get_starter_bottoms().next().unwrap(),
                        shoes: job.get_starter_shoes().next().unwrap(),
                        top: job.get_starter_tops().next().unwrap(),
                        weapon: job.get_starter_weapons().next().unwrap(),
                        guide: job.get_guide_item(),
                    },
                    gender: Gender::Male,
                },
                &self.data.item,
            )
            .await?;

        let job = JobGroup::Legend;
        let char_id = self
            .data
            .char
            .create_character(
                acc_id,
                CharacterCreateDTO {
                    name: "Aran2".to_string(),
                    job_group: JobGroup::Adventurer,
                    face: FaceId::LEISURE_LOOK_M,
                    skin: Skin::Normal,
                    hair: HairId::BLACK_TOBEN,
                    starter_set: ItemStarterSet {
                        bottom: job.get_starter_bottoms().next().unwrap(),
                        shoes: job.get_starter_shoes().next().unwrap(),
                        top: job.get_starter_tops().next().unwrap(),
                        weapon: job.get_starter_weapons().next().unwrap(),
                        guide: job.get_guide_item(),
                    },
                    gender: Gender::Male,
                },
                &self.data.item,
            )
            .await?;

        Ok((acc_id, char_id))
    }
}
