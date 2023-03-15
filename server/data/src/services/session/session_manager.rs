use dashmap::DashMap;
use tokio::sync::Mutex;
use std::{
    hash::Hash, sync::Arc, time::{Instant, Duration},
};


#[async_trait::async_trait]
pub trait Session {
    async fn save() -> anyhow::Result<()>;
}

pub type SessionDataHolder<SessionData> = tokio::sync::OwnedMutexGuard<SessionData>;
pub type MutexSession<SessionData> = Arc<Mutex<SessionData>>;

#[derive(Debug)]
pub struct SessionManager<Key: Eq + Hash, SessionData> {
    sessions: DashMap<Key, MutexSession<SessionData>>
}

impl<Key, SessionData> Default for SessionManager<Key, SessionData> where Key: Eq + Hash {
    fn default() -> Self {
        Self { sessions: Default::default() }
    }
}

impl<Key, SessionData> SessionManager<Key, SessionData>
where
    Key: Eq + Hash + std::fmt::Debug,
    SessionData: Send + 'static,
{
    // TODO: create proper house-cleaning process here and document it
    fn clear_closed_session(&self) {
        let mut held_locks = vec![];

        self.sessions.retain(|_,v| {
            if let Ok(guard) = v.clone().try_lock_owned() {
                held_locks.push(guard);
                false 
            } else {
                true
            }
        });
    }


    pub fn create_session(&self, key: Key, session: SessionData) -> anyhow::Result<()> {
        self.clear_closed_session();
        let mut inserted = false;
        self.sessions.entry(key)
            .or_insert_with(|| {
                inserted = true;
                Arc::new(Mutex::new(session))
            });

        if !inserted {
            anyhow::bail!("Session for key already exists");
        }

        Ok(())
    }

    pub fn create_claim_session(&self, key: Key, session: SessionData) -> anyhow::Result<SessionDataHolder<SessionData>> 
        where Key: Clone {
            self.create_session(key.clone(), session)?;
            self.try_claim_session(&key)
    }

    pub fn try_claim_session(
        &self,
        key: &Key,
    ) -> anyhow::Result<SessionDataHolder<SessionData>> {
        let data = self
            .sessions
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("No session for key"))?
            .value()
            .clone();

        Ok(data.try_lock_owned()?)
    }

    pub async fn try_claim_session_timeout(
        &self,
        key: &Key,
        timeout: Duration
    ) -> anyhow::Result<SessionDataHolder<SessionData>> {
        let data = self
            .sessions
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("No session for key"))?
            .value()
            .clone();

        let now = Instant::now();
        while now.elapsed() < timeout {
            let data = data.clone();
            if let Ok(session) = data.try_lock_owned() {
                return Ok(session)
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        Ok(data.try_lock_owned()?)
    }
}
