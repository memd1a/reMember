use std::hash::Hash;
use std::net::IpAddr;
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
pub struct MigrationService<K, V> where K: Eq + Hash{
    timeout: Duration,
    pending: DashMap<K, MigrationContext<V>>,
}

impl<K, V> Clone for MigrationService<K, V>
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

impl<K, V> MigrationService<K, V>
where
    K: Eq + Hash,
{
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            pending: DashMap::new(),
        }
    }

    pub fn pending(&self) -> usize {
        self.pending.len()
    }

    pub fn take(&self, key: &K) -> Option<V> {
        let ctx = self.pending.remove(key);

        match ctx {
            Some((_, v)) if !v.is_timeout() => Some(v.data),
            _ => None,
        }
    }

    pub fn push(&self, key: K, data: V) {
        //TODO: what to do if there's already a migration data entry for that IP
        self.pending
            .insert(key, MigrationContext::new(data, self.timeout));
    }

    pub fn clean(&self) {
        //TODO figure out how to call clean, capping insert and executing It every x inserts would be nice
        self.pending.retain(|_, v| !v.is_timeout())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct IpIdKey {
    pub ip: IpAddr,
    pub id: u32,
}

impl IpIdKey {
    pub fn new(ip: IpAddr, id: u32) -> Self {
        Self { ip, id }
    }
}

pub type MigrationIpService<V> = MigrationService<IpIdKey, V>;

#[cfg(test)]
mod tests {
    use std::{
        net::{IpAddr, Ipv4Addr, Ipv6Addr},
        time::Duration, thread::sleep,
    };

    use crate::migration::IpIdKey;

    use super::MigrationIpService;

    #[test]
    fn test_insert_remove() {
        const TIMEOUT: Duration = Duration::from_millis(100);
        let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

        let svc = MigrationIpService::<u32>::new(TIMEOUT);

        let key_1 = IpIdKey::new(localhost_v4, 1);
        let key_2 = IpIdKey::new(localhost_v6, 1);


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
