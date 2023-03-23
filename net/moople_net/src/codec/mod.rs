pub mod handshake;
pub mod maple_codec;

pub const MAX_HANDSHAKE_LEN: usize = 24;
pub const MAX_PACKET_LEN: usize = (u16::MAX / 2) as usize;
