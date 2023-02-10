use bytes::{Buf, BufMut};
use moople_packet::{MaplePacket, NetError};
use tokio_util::codec::{Decoder, Encoder, Framed};

use crate::crypto::{header::invert_version, MapleCrypto, PacketHeader, PACKET_HEADER_LEN};

use super::handshake::Handshake;

const MAX_PACKET_LEN: usize = (u16::MAX / 2) as usize;

pub type MapleFramedCodec<T> = Framed<T, MapleCodec>;

pub struct MapleCodec {
    send_version: u16,
    recv_version: u16,
    send_cipher: MapleCrypto,
    recv_cipher: MapleCrypto,
}

impl MapleCodec {
    pub fn client_from_handshake(handshake: &Handshake) -> Self {
        Self {
            send_version: handshake.version,
            recv_version: invert_version(handshake.version),
            send_cipher: MapleCrypto::from_round_key(handshake.iv_enc),
            recv_cipher: MapleCrypto::from_round_key(handshake.iv_dec),
        }
    }

    pub fn server_from_handshake(handshake: &Handshake) -> Self {
        Self {
            send_version: invert_version(handshake.version),
            recv_version: handshake.version,
            send_cipher: MapleCrypto::from_round_key(handshake.iv_dec),
            recv_cipher: MapleCrypto::from_round_key(handshake.iv_enc),
        }
    }
}

impl Decoder for MapleCodec {
    type Item = MaplePacket;
    type Error = NetError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        //Read Header
        if src.len() < PACKET_HEADER_LEN {
            return Ok(None);
        }

        //Decode length
        let hdr_bytes: PacketHeader = src[..PACKET_HEADER_LEN].try_into().unwrap();
        let length = self
            .recv_cipher
            .decode_header(hdr_bytes, self.recv_version)? as usize;

        if length > MAX_PACKET_LEN {
            return Err(NetError::FrameSize(length));
        }

        let total_len = PACKET_HEADER_LEN + length;

        //Read data
        if src.len() < total_len {
            src.reserve(total_len - src.len());
            return Ok(None);
        }

        src.advance(PACKET_HEADER_LEN);
        let mut packet_data = src.split_to(length);
        self.recv_cipher.decrypt(packet_data.as_mut());
        let pkt = MaplePacket::from_data(packet_data.freeze());
        Ok(Some(pkt))
    }
}

impl Encoder<MaplePacket> for MapleCodec {
    type Error = NetError;

    fn encode(&mut self, item: MaplePacket, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let data = item.data;
        let length = data.len();
        if length > MAX_PACKET_LEN {
            return Err(NetError::FrameSize(length));
        }

        let hdr = self
            .send_cipher
            .encode_header(length as u16, self.send_version);
        dst.reserve(PACKET_HEADER_LEN + length);
        dst.put_slice(&hdr);
        dst.put_slice(&data);
        self.send_cipher.encrypt(&mut dst[PACKET_HEADER_LEN..]);
        Ok(())
    }
}
