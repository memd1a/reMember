use dashmap::DashMap;
use std::{
    hash::Hash,
    sync::Arc,
    time::{Duration, Instant}, ops::{Deref, DerefMut},
};
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait SessionBackend {
    type SessionData: std::fmt::Debug;
    type SessionLoadParam;

    async fn load(&self, param: Self::SessionLoadParam) -> anyhow::Result<Self::SessionData>;
    async fn save(&self, session: Self::SessionData) -> anyhow::Result<()>;
}


#[derive(Debug)]
pub struct OwnedSession<Key, SessionData> {
    pub session: tokio::sync::OwnedMutexGuard<SessionData>,
    pub key: Key
}

impl<Key, SessionData> Deref for OwnedSession<Key, SessionData> {
    type Target = SessionData;

    fn deref(&self) -> &Self::Target {
        self.session.deref()
    }
}

impl<Key, SessionData> DerefMut for OwnedSession<Key, SessionData> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.session.deref_mut()
    }
}

pub type SessionMutex<SessionData> = Arc<Mutex<SessionData>>;

#[derive(Debug)]
pub struct SessionManager<Key: Eq + Hash, Backend: SessionBackend> {
    sessions: DashMap<Key, SessionMutex<Backend::SessionData>>,
    backend: Backend
}


impl<Key, Backend> SessionManager<Key, Backend>
where
    Key: Eq + Hash + Clone + std::fmt::Debug,
    Backend: SessionBackend + Send + 'static,
{
    pub fn new(backend: Backend) -> Self {
        Self {
            sessions: DashMap::new(),
            backend
        }
    }


    // TODO: create proper house-cleaning process here and document it
    fn clear_closed_session(&self) {
        let mut held_locks = vec![];

        self.sessions.retain(|_, v| {
            if let Ok(guard) = v.clone().try_lock_owned() {
                held_locks.push(guard);
                false
            } else {
                true
            }
        });
    }

    fn create_session_from_data(&self, key: Key, data: Backend::SessionData) -> anyhow::Result<()> {
        let mut inserted = false;
        self.sessions.entry(key).or_insert_with(|| {
            inserted = true;
            Arc::new(Mutex::new(data))
        });

        if !inserted {
            anyhow::bail!("Session for key already exists");
        }

        Ok(())
    }

    pub async fn close_session(&self, session: OwnedSession<Key, Backend::SessionData>) -> anyhow::Result<()> {
        let key = session.key.clone();
        //self.backend.save(session).await?;
        
        // Release lock
        drop(session);

        // Remove session
        let session = self.sessions.remove(&key).unwrap();

        let session = Arc::<tokio::sync::Mutex<<Backend as SessionBackend>::SessionData>>::try_unwrap(session.1).unwrap();
        let session_data = session.into_inner();
        self.backend.save(session_data).await?;


        Ok(())
    }

    pub async fn create_session(&self, key: Key, param: Backend::SessionLoadParam) -> anyhow::Result<()> {
        self.clear_closed_session();

        let data = self.backend.load(param).await?;
        self.create_session_from_data(key, data)
    }

    pub async fn create_claim_session(
        &self,
        key: Key,
        param: Backend::SessionLoadParam
    ) -> anyhow::Result<OwnedSession<Key, Backend::SessionData>>
    where
        Key: Clone,
    {
        self.create_session(key.clone(), param).await?;
        self.try_claim_session(&key)
    }

    pub fn try_claim_session(&self, key: &Key) -> anyhow::Result<OwnedSession<Key, Backend::SessionData>> {
        let data = self
            .sessions
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("No session for key"))?
            .value()
            .clone();

        Ok(OwnedSession {
            session: data.try_lock_owned()?,
            key: key.clone()
        })
    }

    pub async fn try_claim_session_timeout(
        &self,
        key: &Key,
        timeout: Duration,
    ) -> anyhow::Result<OwnedSession<Key, Backend::SessionData>> {
        let data = self
            .sessions
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("No session for key"))?
            .value()
            .clone();

        let now = Instant::now();
        while now.elapsed() < timeout {
            if let Ok(session) = data.clone().try_lock_owned() {
                return Ok(OwnedSession {
                    session,
                    key: key.clone()
                });
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        anyhow::bail!("Timeout")
    }
}
