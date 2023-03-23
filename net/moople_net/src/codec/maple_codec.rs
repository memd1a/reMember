use bytes::Buf;
use moople_packet::{MaplePacket, NetError, NetResult};
use tokio_util::codec::{Decoder, Encoder, Framed};

use crate::crypto::{MapleCrypto, PacketHeader, PACKET_HEADER_LEN, MapleVersion};

use super::{handshake::Handshake, MAX_PACKET_LEN};

pub type MapleFramedCodec<T> = Framed<T, MapleCodec>;

// Struct for the encoder which ensures only this crate can actually use it
// Because the encoding has to be done prior with the raw encode buffer
// Check `MapleSession::send_packet` for this
pub struct EncodeItem(pub(crate) usize);

pub struct MapleCodec {
    send_cipher: MapleCrypto,
    recv_cipher: MapleCrypto,
}

impl MapleCodec {
    pub fn client_from_handshake(handshake: &Handshake) -> Self {
        let v = MapleVersion(handshake.version);
        Self {
            send_cipher: MapleCrypto::from_round_key(handshake.iv_enc, v),
            recv_cipher: MapleCrypto::from_round_key(
                handshake.iv_dec,
                v.invert(),
            ),
        }
    }

    pub fn server_from_handshake(handshake: &Handshake) -> Self {
        let v = MapleVersion(handshake.version);
        Self {
            send_cipher: MapleCrypto::from_round_key(
                handshake.iv_dec,
                v.invert(),
            ),
            recv_cipher: MapleCrypto::from_round_key(handshake.iv_enc, v),
        }
    }
}

fn check_packet_len(len: usize) -> NetResult<()> {
    if len > MAX_PACKET_LEN {
        return Err(NetError::FrameSize(len));
    }

    Ok(())
}

impl Decoder for MapleCodec {
    type Item = MaplePacket;
    type Error = NetError;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < PACKET_HEADER_LEN {
            return Ok(None);
        }
        let hdr: PacketHeader = src[..PACKET_HEADER_LEN].try_into().unwrap();
        let length = self.recv_cipher.decode_header(hdr)? as usize;

        // Verify the packet is not great than the maximum limit
        check_packet_len(length)?;

        // Try to read the actual payload
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

impl Encoder<EncodeItem> for MapleCodec {
    type Error = NetError;

    fn encode(&mut self, item: EncodeItem, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let length = item.0 - PACKET_HEADER_LEN;
        check_packet_len(length)?;

        let hdr = self.send_cipher.encode_header(length as u16);
        dst[..PACKET_HEADER_LEN].copy_from_slice(hdr.as_slice());
        self.send_cipher.encrypt(&mut dst[PACKET_HEADER_LEN..]);
        Ok(())
    }
}
