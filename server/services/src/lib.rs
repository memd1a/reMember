pub mod account;
pub mod character;
pub mod helper;
pub mod item;
pub mod migration;
pub mod model;
pub mod server_info;
pub mod util;
use std::{net::IpAddr, sync::Arc, time::Duration};

use account::{AccountId, AccountService, Region};
use character::{CharacterCreateDTO, CharacterID, CharacterService, ItemStarterSet};
use item::ItemService;
use migration::MigrationIpService;
use proto95::{
    id::{job_id::JobGroup, FaceId, HairId, Skin},
    shared::Gender,
};
use sea_orm::{DatabaseConnection, DbErr};
use server_info::{ServerInfo, ServerService};

pub type SharedServices = Arc<Services>;

#[derive(Debug, Clone)]
pub struct MigrationSessionContext {
    pub client_ip: IpAddr,
    pub acc_id: u32,
    pub char_id: u32,
}

#[derive(Debug)]
pub struct Services {
    pub account: AccountService,
    pub character: CharacterService,
    pub item: ItemService,
    pub migration: MigrationIpService<MigrationSessionContext>,
    pub server_info: ServerService,
}

impl Services {
    pub fn new(db: DatabaseConnection, servers: impl IntoIterator<Item = ServerInfo>) -> Self {
        Self {
            account: AccountService::new(db.clone()),
            item: ItemService::new(db.clone()),
            character: CharacterService::new(db),
            migration: MigrationIpService::new(Duration::from_secs(30)),
            server_info: ServerService::new(servers),
        }
    }

    pub async fn seeded_in_memory(
        servers: impl IntoIterator<Item = ServerInfo>,
    ) -> Result<Self, DbErr> {
        let db = data::gen_sqlite(data::SQL_OPT_MEMORY).await?;
        Ok(Self::new(db, servers))
    }

    pub fn as_shared(self) -> SharedServices {
        Arc::new(self)
    }

    pub async fn seed_acc_char(&self) -> anyhow::Result<(AccountId, CharacterID)> {
        let acc_id = self
            .account
            .create("admin", "test123", Region::Europe, true)
            .await?;

        let job = JobGroup::Legend;
        let char_id = self
            .character
            .create_character(
                acc_id,
                CharacterCreateDTO {
                    name: "Aran".to_string(),
                    job_group: JobGroup::Legend,
                    face: FaceId::FEARFUL_STARE_F,
                    skin: Skin::White,
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
            )
            .await?;

        Ok((acc_id, char_id))
    }
}
