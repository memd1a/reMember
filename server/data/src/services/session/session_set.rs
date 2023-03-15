use std::sync::Arc;

use dashmap::DashMap;
use moople_packet::{MaplePacket, EncodePacket, HasOpcode, MaplePacketWriter};
use tokio::sync::mpsc;

use crate::services::character::CharacterID;

/*
    TODO:
        * allow using pressured send for broadcast
        * work out the buffer situation for broadcasts and unicasts

        In theory each session could have their own encoding buffer for unicast packets
        and the session set could have one encode buffer

        HOWEVER allocation and the buffer being unavailable has to be worked with corretelcy

 */

#[derive(Debug, Clone)]
pub struct SharedSessionData {
    pub broadcast_tx: mpsc::Sender<MaplePacket>,
}

pub type SharedSessionDataRef = Arc<SharedSessionData>;

#[derive(Debug)]
pub struct SessionSet {
    sessions: DashMap<CharacterID, SharedSessionDataRef>,
}

impl SessionSet {
    pub fn new() -> Self {
        Self {
            sessions: DashMap::new(),
        }
    }

    pub fn add(&self, key: CharacterID, session: SharedSessionDataRef) {
        self.sessions.insert(key, session);
    }

    pub fn remove(&self, key: CharacterID) {
        self.sessions.remove(&key);
    }

    pub async fn send_packet_to(
        &self,
        rx_key: CharacterID,
        pkt: MaplePacket,
    ) -> anyhow::Result<()> {
        self.sessions
            .get(&rx_key)
            .ok_or_else(|| anyhow::format_err!("Unable to find session"))?
            .broadcast_tx
            .send(pkt)
            .await?;

        Ok(())
    }

    pub async fn broadcast_packet(&self, pkt: MaplePacket, src: CharacterID) -> anyhow::Result<()> {
        // Self broadcast not allowed
        for r in self.sessions.iter() {
            if *r.key() != src {
                r.broadcast_tx.send(pkt.clone()).await?;
            }
        }

        Ok(())
    }

    pub async fn broadcast_pkt<T: EncodePacket + HasOpcode>(&self, pkt: T, src: CharacterID) -> anyhow::Result<()> {
        let mut pw = MaplePacketWriter::default();
        pw.write_opcode(T::OPCODE);
        pkt.encode_packet(&mut pw)?;

        self.broadcast_packet(pw.into_packet(), src).await
    }

    pub async fn send_pkt_to<T: EncodePacket + HasOpcode>(&self, rx_key: CharacterID, pkt: T) -> anyhow::Result<()> {
        let mut pw = MaplePacketWriter::default();
        pw.write_opcode(T::OPCODE);
        pkt.encode_packet(&mut pw)?;

        self.send_packet_to(rx_key, pw.into_packet()).await
    }
}
