use std::hash::Hash;
use std::time::{Duration, Instant};

use dashmap::DashMap;

#[derive(Debug, Clone)]
struct MigrationContext<V> {
    data: V,
    timeout: Instant,
}

impl<V> MigrationContext<V> {
    pub fn new(data: V, timeout_dur: Duration) -> Self {
        Self {
            data,
            timeout: Instant::now() + timeout_dur,
        }
    }

    pub fn is_timeout(&self) -> bool {
        self.timeout < Instant::now()
    }
}

#[derive(Debug)]
pub struct MigrationManager<K, V>
where
    K: Eq + Hash,
{
    timeout: Duration,
    pending: DashMap<K, MigrationContext<V>>,
}

impl<K, V> Clone for MigrationManager<K, V>
where
    K: Clone + Hash + Eq,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            timeout: self.timeout,
            pending: self.pending.clone(),
        }
    }
}

impl<K, V> MigrationManager<K, V>
where
    K: Eq + Hash,
{
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            pending: DashMap::default(),
        }
    }

    pub fn pending(&self) -> usize {
        self.pending.len()
    }

    pub async fn take_timeout(&self, key: &K) -> anyhow::Result<V> {
        let start = Instant::now();
        //TODO this should be handled better
        while start.elapsed() < self.timeout {
            if let Some((_, ctx)) = self.pending.remove(key) {
                if !ctx.is_timeout() {
                    return Ok(ctx.data);
                } else {
                    // If timeout is reached the session handle will be dropped
                    break;
                }
            };

            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        anyhow::bail!("Timeout reached for migration");
    }

    pub fn take(&self, key: &K) -> Option<V> {
        let ctx = self.pending.remove(key);

        match ctx {
            Some((_, v)) if !v.is_timeout() => Some(v.data),
            _ => None,
        }
    }

    pub fn push(&self, key: K, data: V) {
        //TODO: what to do if there's already a migration data entry for that key
        self.pending
            .insert(key, MigrationContext::new(data, self.timeout));
    }

    pub fn clean(&self) {
        //TODO figure out how to call clean, capping insert and executing It every x inserts would be nice
        self.pending.retain(|_, v| !v.is_timeout())
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::services::session::migration::MigrationManager;

    #[test]
    fn test_insert_remove() {
        const TIMEOUT: Duration = Duration::from_millis(100);

        let svc = MigrationManager::<u32, u32>::new(TIMEOUT);

        let key_1 = 1;
        let key_2 = 2;

        // Test insert/remove
        assert_eq!(svc.take(&key_1), None);
        svc.push(key_1, 10);
        assert_eq!(svc.take(&key_1), Some(10));
        assert_eq!(svc.take(&key_1), None);
        assert_eq!(svc.take(&key_2), None);

        //Test timeout
        svc.push(key_1, 10);
        assert_eq!(svc.pending(), 1);
        sleep(TIMEOUT * 2);
        assert_eq!(svc.take(&key_1), None);

        // Test clean
        svc.push(key_1, 10);
        assert_eq!(svc.pending(), 1);
        sleep(TIMEOUT * 2);
        assert_eq!(svc.pending(), 1);
        svc.clean();
        assert_eq!(svc.pending(), 0);
    }
}
