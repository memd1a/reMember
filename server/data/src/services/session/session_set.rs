use std::sync::Arc;

use dashmap::DashMap;
use moople_packet::{MaplePacket, EncodePacket, HasOpcode, MaplePacketWriter};
use tokio::sync::{mpsc, broadcast};

use crate::services::data::character::CharacterID;

/*
    TODO:
        * allow using pressured send for broadcast
        * work out the buffer situation for broadcasts and unicasts

        In theory each session could have their own encoding buffer for unicast packets
        and the session set could have one encode buffer

        HOWEVER allocation and the buffer being unavailable has to be worked with corretelcy

 */


pub type BroadcastPacket = (CharacterID, MaplePacket);
pub type BroadcastRx = broadcast::Receiver<BroadcastPacket>;

#[derive(Debug, Clone)]
pub struct SharedSessionData {
    pub session_tx: mpsc::Sender<MaplePacket>,
}

pub type SharedSessionDataRef = Arc<SharedSessionData>;

impl SharedSessionData {
    pub async fn send_pkt(&self, pkt: MaplePacket) -> anyhow::Result<()> {
        self.session_tx.send(pkt).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct SessionSet {
    sessions: DashMap<CharacterID, SharedSessionDataRef>,
    broadcast_tx: broadcast::Sender<BroadcastPacket>,
    _broadcast_rx: broadcast::Receiver<BroadcastPacket>,

}

impl SessionSet {
    pub fn new() -> Self {
        let (broadcast_tx, _broadcast_rx) = broadcast::channel(64);
        Self {
            sessions: DashMap::new(),
            broadcast_tx,
            _broadcast_rx,
        }
    }

    pub fn add(&self, key: CharacterID, session: SharedSessionDataRef) -> BroadcastRx {
        self.sessions.insert(key, session);
        self.broadcast_tx.subscribe()
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
            .session_tx
            .send(pkt)
            .await?;

        Ok(())
    }

    pub fn broadcast_packet(&self, pkt: MaplePacket, src: CharacterID) -> anyhow::Result<()> {
        self.broadcast_tx.send((src, pkt))?;
        Ok(())
    }

    pub fn broadcast_pkt<T: EncodePacket + HasOpcode>(&self, pkt: T, src: CharacterID) -> anyhow::Result<()> {
        let mut pw = MaplePacketWriter::default();
        pw.write_opcode(T::OPCODE);
        pkt.encode_packet(&mut pw)?;

        self.broadcast_packet(pw.into_packet(), src)?;
        Ok(())
    }

    pub async fn send_pkt_to<T: EncodePacket + HasOpcode>(&self, rx_key: CharacterID, pkt: T) -> anyhow::Result<()> {
        let mut pw = MaplePacketWriter::default();
        pw.write_opcode(T::OPCODE);
        pkt.encode_packet(&mut pw)?;

        self.send_packet_to(rx_key, pw.into_packet()).await
    }
}
