pub mod reactor;
pub mod user;
pub mod drop;
pub mod mob;
pub mod npc;

pub use drop::Drop;

use itertools::Itertools;
pub use mob::Mob;
pub use npc::Npc;
use tokio::sync::RwLock;

use std::{collections::BTreeMap, sync::atomic::AtomicU32};

use moople_packet::{EncodePacket, HasOpcode, MaplePacketWriter};
use proto95::game::ObjectId;
use std::fmt::Debug;

use crate::services::session::session_set::{SessionSet, SharedSessionDataRef};

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
}

impl<T> Pool<T>
where
    T: PoolItem<Id = ObjectId>,
{
    pub fn new() -> Self {
        Self {
            items: RwLock::new(BTreeMap::new()),
            next_id: AtomicU32::new(0),
        }
    }

    pub fn next_id(&self) -> u32 {
        self.next_id
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    pub fn from_elems(elems: impl Iterator<Item = T>) -> Self {
        let pool = Pool::new();
        {
            let mut items = pool.items.try_write().unwrap();
            items.extend(elems.map(|item| (pool.next_id(), item)));
        }
        pool
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
    ) -> anyhow::Result<()> {
        let Some(item) = self.items.write().await.remove(&id) else {
            anyhow::bail!("Item does not exist");
        };

        let pkt = item.get_leave_pkt(id, param);
        sessions.broadcast_pkt(pkt, -1).await?;
        Ok(())
    }

    pub async fn on_enter(&self, session: SharedSessionDataRef) -> anyhow::Result<()> {
        let broadcast_packets = self
            .items
            .read()
            .await
            .iter()
            .map(|(id, v)| v.get_enter_pkt(*id))
            .collect_vec();

        for pkt in broadcast_packets {
            let mut pw = MaplePacketWriter::default();
            pw.write_opcode(T::EnterPacket::OPCODE);
            pkt.encode_packet(&mut pw)?;
            session.broadcast_tx.send(pw.into_packet()).await?;
        }

        Ok(())
    }
}
