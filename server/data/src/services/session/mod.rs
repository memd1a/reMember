pub mod session_set;
pub mod migration;
pub mod session_data;
pub mod session_manager;

use std::net::IpAddr;
use std::time::Duration;

use self::{migration::MigrationManager, session_manager::SessionManager, session_data::MoopleSessionHolder};

use super::{MoopleSessionData};

// Client uses a 8 byte session id
pub type ClientKey = [u8; 8];

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct MoopleMigrationKey {
    client_key: ClientKey,
    peer_addr: IpAddr,
}

impl MoopleMigrationKey {
    pub fn new(client_key: ClientKey, peer_addr: IpAddr) -> Self {
        Self {
            client_key,
            peer_addr,
        }
    }
}

#[derive(Debug)]
pub struct GameSessionManager {
    session_man: SessionManager<uuid::Uuid, MoopleSessionData>,
    migration: MigrationManager<MoopleMigrationKey, MoopleSessionHolder>,
}

impl GameSessionManager {
    pub fn new(migration_timeout: Duration) -> Self {
        GameSessionManager {
            session_man: SessionManager::default(),
            migration: MigrationManager::new(migration_timeout),
        }
    }

    pub fn create_migration_session(
        &self,
        migration_key: MoopleMigrationKey,
        session: MoopleSessionData,
    ) -> anyhow::Result<()> {
        let session = self
            .session_man
            .create_claim_session(uuid::Uuid::new_v4(), session)?;
        self.migration.push(migration_key, session);
        Ok(())
    }

    pub fn migrate_session(
        &self,
        migration_key: MoopleMigrationKey,
        session: MoopleSessionHolder,
    ) -> anyhow::Result<()> {
        self.migration.push(migration_key, session);
        Ok(())
    }

    pub async fn claim_migration_session(
        &self,
        migration_key: MoopleMigrationKey,
    ) -> anyhow::Result<MoopleSessionHolder> {
        self.migration.take_timeout(&migration_key).await
    }
}
