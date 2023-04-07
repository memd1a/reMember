use std::{collections::BTreeMap, sync::RwLock};

use moople_packet::{EncodePacket, HasOpcode, MaplePacket, MaplePacketWriter};

use super::session_svc::SharedSessionHandle;

#[derive(Debug)]
pub struct SessionSet<Key>(RwLock<BTreeMap<Key, SharedSessionHandle>>);

impl<Key: Eq + Ord> Default for SessionSet<Key> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Key: Eq + Ord> SessionSet<Key> {
    pub fn new() -> Self {
        Self(RwLock::default())
    }

    pub fn add(&self, key: Key, session: SharedSessionHandle) {
        self.0.write().expect("Session add").insert(key, session);
    }

    pub fn remove(&self, key: Key) {
        self.0.write().expect("Session remove").remove(&key);
    }

    pub fn send_packet_to(&self, rx_key: Key, pkt: MaplePacket) -> anyhow::Result<()> {
        self.0
            .read()
            .expect("Session send to")
            .get(&rx_key)
            .ok_or_else(|| anyhow::format_err!("Unable to find session"))?
            .tx
            .clone()
            .try_send(pkt.data)?;

        Ok(())
    }

    pub fn broadcast_packet(&self, pkt: MaplePacket, src: Key) -> anyhow::Result<()> {
        for (key, sess) in self.0.read().expect("Session broadcast ").iter() {
            if src == *key {
                continue;
            }
            let _ = sess.tx.clone().try_send(&pkt.data);
        }
        Ok(())
    }

    pub fn broadcast_pkt<T: EncodePacket + HasOpcode>(
        &self,
        pkt: T,
        src: Key,
    ) -> anyhow::Result<()> {
        let mut pw = MaplePacketWriter::default();
        pw.write_opcode(T::OPCODE);
        pkt.encode_packet(&mut pw)?;

        self.broadcast_packet(pw.into_packet(), src)?;
        Ok(())
    }

    pub async fn send_pkt_to<T: EncodePacket + HasOpcode>(
        &self,
        rx_key: Key,
        pkt: T,
    ) -> anyhow::Result<()> {
        let mut pw = MaplePacketWriter::default();
        pw.write_opcode(T::OPCODE);
        pkt.encode_packet(&mut pw)?;

        self.send_packet_to(rx_key, pw.into_packet())
    }
}
