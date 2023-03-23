pub mod drop;
pub mod mob;
pub mod npc;
pub mod reactor;
pub mod user;

pub use drop::Drop;

pub use mob::Mob;
use moople_net::service::packet_buffer::PacketBuffer;
pub use npc::Npc;
use tokio::sync::RwLock;

use std::{collections::BTreeMap, sync::atomic::AtomicU32};

use moople_packet::{EncodePacket, HasOpcode};
use proto95::game::ObjectId;
use std::fmt::Debug;

use crate::services::{meta::meta_service::MetaService, session::session_set::SessionSet};

pub trait PoolId {}

pub trait PoolItem {
    type Id: Clone + Eq;
    type EnterPacket: EncodePacket + HasOpcode;
    type LeavePacket: EncodePacket + HasOpcode;
    type LeaveParam;

    fn get_enter_pkt(&self, id: Self::Id) -> Self::EnterPacket;
    fn get_leave_pkt(&self, id: Self::Id, param: Self::LeaveParam) -> Self::LeavePacket;
}

pub fn next_id() -> ObjectId {
    static ID: AtomicU32 = AtomicU32::new(0);
    ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

#[derive(Debug)]
pub struct Pool<T>
where
    T: PoolItem<Id = ObjectId>,
{
    items: RwLock<BTreeMap<T::Id, T>>,
    next_id: AtomicU32,
    meta: &'static MetaService,
}

impl<T> Pool<T>
where
    T: PoolItem<Id = ObjectId>,
{
    pub fn new(meta: &'static MetaService) -> Self {
        Self {
            items: RwLock::new(BTreeMap::new()),
            next_id: AtomicU32::new(0),
            meta,
        }
    }

    pub fn next_id(&self) -> u32 {
        self.next_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn from_elems(meta: &'static MetaService, elems: impl Iterator<Item = T>) -> Self {
        let pool = Pool::new(meta);
        {
            let mut items = pool.items.try_write().unwrap();
            items.extend(elems.map(|item| (pool.next_id(), item)));
        }
        pool
    }

    pub async fn update(&self, id: ObjectId, update: impl Fn(&mut T)) {
        let mut items = self.items.write().await;
        if let Some(item) = items.get_mut(&id) {
            update(item);
        }
    }

    pub async fn add(&self, item: T, sessions: &SessionSet) -> anyhow::Result<()> {
        let id = self.next_id();
        let pkt = item.get_enter_pkt(id.clone());
        self.items.write().await.insert(id, item);

        sessions.broadcast_pkt(pkt, -1).await?;
        Ok(())
    }

    pub async fn remove(
        &self,
        id: T::Id,
        param: T::LeaveParam,
        sessions: &SessionSet,
    ) -> anyhow::Result<T> {
        let Some(item) = self.items.write().await.remove(&id) else {
            anyhow::bail!("Item does not exist");
        };

        let pkt = item.get_leave_pkt(id, param);
        sessions.broadcast_pkt(pkt, -1).await?;
        Ok(item)
    }

    pub async fn on_enter(&self, packet_buf: &mut PacketBuffer) -> anyhow::Result<()> {
        for (id, pkt) in self.items.read().await.iter() {
            packet_buf.write_packet(pkt.get_enter_pkt(*id))?;
        }

        Ok(())
    }
}