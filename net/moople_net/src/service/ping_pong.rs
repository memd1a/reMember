use std::time::{Duration, Instant};

use moople_packet::{MaplePacket, EncodePacket, HasOpcode, MaplePacketWriter};

pub struct PingPongConfig {
    pub interval: Duration,
    pub timeout: Duration,
}

impl Default for PingPongConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(4 * 30),
        }
    }
}

pub trait PingPongHandler {
    fn get_ping_packet(&self) -> MaplePacket;
    fn get_pong_packet(&self) -> MaplePacket;    

    fn handle_update(&mut self);
    fn is_timeout(&self) -> bool;
    fn update_interval(&self) -> Duration;
}
pub struct PacketPingPongHandler<Ping, Pong> {
    ping_packet: Ping,
    pong_packet: Pong,
    cfg: PingPongConfig,
    last_update: Instant
}

impl<Ping, Pong> PacketPingPongHandler<Ping, Pong> {
    pub fn new(ping_pkt: Ping, pong_pkt: Pong, cfg: PingPongConfig) -> Self {
        Self {
            ping_packet: ping_pkt,
            pong_packet: pong_pkt,
            cfg,
            last_update: Instant::now()
        }
    }
}


impl<Ping, Pong> PingPongHandler for PacketPingPongHandler<Ping, Pong> where Ping: EncodePacket + HasOpcode, Pong: EncodePacket + HasOpcode {
    fn get_ping_packet(&self) -> MaplePacket {
        let mut pw = MaplePacketWriter::default();
        self.ping_packet.encode_packet(&mut pw).expect("Ping packet encode must work");

        pw.into_packet()
    }

    fn get_pong_packet(&self) -> MaplePacket {
        let mut pw = MaplePacketWriter::default();
        self.pong_packet.encode_packet(&mut pw).expect("Pong packet encode must work");
        pw.into_packet()
    }

    fn is_timeout(&self) -> bool {
        self.last_update.elapsed() >= self.cfg.timeout
    }

    fn handle_update(&mut self) {
        self.last_update = Instant::now()

    }

    fn update_interval(&self) -> Duration {
        self.cfg.interval
    }
}