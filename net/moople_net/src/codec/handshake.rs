use std::io::{Read, Write};

use moople_packet::{
    DecodePacket, EncodePacket, MaplePacketWriter, NetError, PacketLen, proto::wrapped::MapleWrapped,
};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{
    crypto::{RoundKey, ROUND_KEY_LEN},
    NetResult,
};

use super::MAX_HANDSHAKE_LEN;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Handshake {
    pub version: u16,
    pub subversion: String,
    pub iv_enc: RoundKey,
    pub iv_dec: RoundKey,
    pub locale: u8,
}

impl Handshake {
    fn decode_handshake_len(data: [u8; 2]) -> NetResult<usize> {
        let ln = u16::from_le_bytes(data) as usize;
        if ln <= MAX_HANDSHAKE_LEN {
            Ok(ln)
        } else {
            Err(NetError::HandshakeSize(ln))
        }
    }

    pub async fn read_handshake_async<R: AsyncRead + Unpin>(mut r: R) -> NetResult<Self> {
        let mut ln_data = [0u8; 2];
        r.read_exact(&mut ln_data).await?;
        let ln = Self::decode_handshake_len(ln_data)?;

        let mut handshake_data = [0u8; MAX_HANDSHAKE_LEN];
        r.read_exact(&mut handshake_data[..ln]).await?;
        Self::decode_from_data(&handshake_data[..ln]).map_err(|_| NetError::InvalidHandshake)
    }

    pub fn read_handshake<R: Read>(mut r: R) -> NetResult<Self> {
        let mut ln_data = [0u8; 2];
        r.read_exact(&mut ln_data)?;
        let ln = Self::decode_handshake_len(ln_data)?;

        let mut handshake_data = [0u8; MAX_HANDSHAKE_LEN];
        r.read_exact(&mut handshake_data[..ln])?;
        Self::decode_from_data(&handshake_data[..ln]).map_err(|_| NetError::InvalidHandshake)
    }

    pub fn write_handshake<W: Write>(&self, mut w: W) -> NetResult<()> {
        let (data, n) = self.encode_with_len();
        w.write_all(&data[..n])?;

        Ok(())
    }

    pub async fn write_handshake_async<W: AsyncWrite + Unpin>(&self, mut w: W) -> NetResult<()> {
        let (data, n) = self.encode_with_len();
        w.write_all(&data[..n]).await?;

        Ok(())
    }

    pub fn encode_with_len(&self) -> ([u8; MAX_HANDSHAKE_LEN], usize) {
        let n = self.packet_len();

        //TODO return buffer with size somehow, use arrayvec? and impl bufmut?
        let mut data = [0; MAX_HANDSHAKE_LEN];
        let mut pw = MaplePacketWriter::new(data.as_mut());
        pw.write_u16(n as u16);
        self.encode_packet(&mut pw).unwrap();

        (data, n + 2)
    }
}

impl MapleWrapped for Handshake {
    type Inner = (u16, String, [u8; ROUND_KEY_LEN], [u8; ROUND_KEY_LEN], u8);

    fn maple_into_inner(&self) -> Self::Inner {
        (
            self.version,
            self.subversion.clone(),
            self.iv_enc.0,
            self.iv_dec.0,
            self.locale
        )
    }

    fn maple_from(v: Self::Inner) -> Self {
        Self{
            version: v.0,
            subversion: v.1,
            iv_enc: RoundKey(v.2),
            iv_dec: RoundKey(v.3),
            locale: v.4
        }
    }
}

#[cfg(test)]
mod tests {
    use moople_packet::{DecodePacket, EncodePacket, MaplePacket, MaplePacketWriter};

    use crate::crypto::RoundKey;

    use super::Handshake;

    #[test]
    fn test_handshake_encode_decode() {
        let handshake = Handshake {
            version: 1,
            subversion: "2".to_string(),
            iv_enc: RoundKey([1u8; 4]),
            iv_dec: RoundKey([2u8; 4]),
            locale: 5,
        };

        let mut pw = MaplePacketWriter::default();
        handshake.encode_packet(&mut pw).unwrap();
        let pkt = MaplePacket::from_writer(pw);
        let mut pr = pkt.into_reader();
        let dec = Handshake::decode_packet(&mut pr).unwrap();

        assert_eq!(handshake, dec);
    }
}
