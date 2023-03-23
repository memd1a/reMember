pub mod framed_pipe;
pub mod packet_buffer;

use arrayvec::ArrayString;

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
    sub_version: ArrayString<2>,
    locale: u8,
}

impl BasicHandshakeGenerator {
    pub fn new(version: u16, sub_version: &str, locale: u8) -> Self {
        Self {
            version,
            sub_version: sub_version.try_into().expect("Subversion"),
            locale,
        }
    }

    pub fn v95() -> Self {
        Self::new(95, "1", 8)
    }

    pub fn v83() -> Self {
        Self::new(83, "1", 8)
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
