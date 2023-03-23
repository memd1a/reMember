pub mod packet_buffer;
use crate::{codec::handshake::Handshake, crypto::RoundKey};

pub mod handler;
pub mod ping_pong;
pub mod resp;
pub mod session_svc;

pub trait HandshakeGenerator {
    fn generate_handshake(&self) -> Handshake;
}

#[derive(Debug, Clone)]
pub struct BasicHandshakeGenerator {
    version: u16,
    sub_version: String,
    locale: u8,
}

impl BasicHandshakeGenerator {
    pub fn new(version: u16, sub_version: String, locale: u8) -> Self {
        Self {
            version,
            sub_version,
            locale,
        }
    }

    pub fn v95() -> Self {
        Self::new(95, "1".to_string(), 8)
    }

    pub fn v83() -> Self {
        Self::new(83, "1".to_string(), 8)
    }
}

impl HandshakeGenerator for BasicHandshakeGenerator {
    fn generate_handshake(&self) -> Handshake {
        let mut rng = rand::thread_rng();
        Handshake {
            version: self.version,
            subversion: self.sub_version.clone(),
            iv_enc: RoundKey::get_random(&mut rng),
            iv_dec: RoundKey::get_random(&mut rng),
            locale: self.locale,
        }
    }
}
