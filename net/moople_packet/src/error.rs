use std::{io, str::Utf8Error};

use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use thiserror::Error;

use crate::analyzer::PacketDataAnalytics;


#[derive(Debug, Error)]
pub enum NetError {
    #[error("IO")]
    IO(#[from] io::Error),
    #[error("string utf8 error")]
    StringUtf8(#[from] Utf8Error),
    #[error("EOF error during reading a packet(type={type_name}): {analytics}")]
    EOF {
        analytics: Box<PacketDataAnalytics>,
        type_name: &'static str,
    },
    #[error("String limit {0} exceeed")]
    StringLimit(usize),
    #[error("Invalid header with key: {key:X}, expected: {expected_key:X}, len: {len}")]
    InvalidHeader {
        len: u16,
        key: u16,
        expected_key: u16,
    },
    #[error("Invalid enum discriminant {0}")]
    InvalidEnumDiscriminant(usize),
    #[error("Invalid enum primitive {0}")]
    InvalidEnumPrimitive(u32),
    #[error("Frame of length {0} is too large.")]
    FrameSize(usize),
    #[error("Handshake of length {0} is too large.")]
    HandshakeSize(usize),
    #[error("Unable to read handshake")]
    InvalidHandshake,
    #[error("Invalid AES key")]
    InvalidAESKey,
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(i64),
    #[error("Invalid opcode: {0:X}")]
    InvalidOpcode(u16),
    #[error("Migrated")]
    Migrated
}

impl NetError {
    pub fn eof<T>(data: &[u8], read_len: usize) -> Self {
        let type_name = std::any::type_name::<T>();
        let pos = data.len().saturating_sub(read_len);

        Self::EOF {
            analytics: Box::new(PacketDataAnalytics::from_data(
                data,
                pos,
                read_len,
                read_len * 5,
            )),
            type_name,
        }
    }
}

impl<E> From<TryFromPrimitiveError<E>> for NetError
where
    E: TryFromPrimitive,
    E::Primitive: Into<u32>,
{
    fn from(value: TryFromPrimitiveError<E>) -> Self {
        NetError::InvalidEnumPrimitive(value.number.into())
    }
}
