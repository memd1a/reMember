pub mod migration;
pub mod session_data;
pub mod session_manager;
use std::net::IpAddr;
use std::time::Duration;

use moople_net::service::session_set::SessionSet;

use self::{
    migration::MigrationManager,
    session_manager::{SessionBackend, OwnedSession, SessionManager},
};

use super::data::character::CharacterID;

pub type MoopleSessionSet = SessionSet<CharacterID>;

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
pub struct GameSessionManager<Backend: SessionBackend> {
    session_man: SessionManager<uuid::Uuid, Backend>,
    migration: MigrationManager<MoopleMigrationKey, OwnedSession<uuid::Uuid, Backend::SessionData>>,
}

impl<Backend> GameSessionManager<Backend>
where
    Backend: SessionBackend + Send + 'static,
{
    pub fn new(backend: Backend, migration_timeout: Duration) -> Self {
        GameSessionManager {
            session_man: SessionManager::new(backend),
            migration: MigrationManager::new(migration_timeout),
        }
    }

    pub async fn close_session(&self, session: OwnedSession<uuid::Uuid, Backend::SessionData>) -> anyhow::Result<()> {
        self.session_man.close_session(session).await
    }

    pub async fn create_migration_session(
        &self,
        migration_key: MoopleMigrationKey,
        param: Backend::SessionLoadParam,
    ) -> anyhow::Result<()> {
        let session = self
            .session_man
            .create_claim_session(uuid::Uuid::new_v4(), param)
            .await?;
        self.migration.push(migration_key, session);
        Ok(())
    }

    pub fn migrate_session(
        &self,
        migration_key: MoopleMigrationKey,
        session: OwnedSession<uuid::Uuid, Backend::SessionData>,
    ) -> anyhow::Result<()> {
        self.migration.push(migration_key, session);
        Ok(())
    }

    pub async fn claim_migration_session(
        &self,
        migration_key: MoopleMigrationKey,
    ) -> anyhow::Result<OwnedSession<uuid::Uuid, Backend::SessionData>> {
        self.migration.take_timeout(&migration_key).await
    }
}
