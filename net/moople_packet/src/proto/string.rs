use bytes::BufMut;

use crate::{DecodePacket, EncodePacket, MaplePacketReader, MaplePacketWriter, NetResult};

use super::PacketLen;

impl<const N: usize> EncodePacket for arrayvec::ArrayString<N> {
    fn encode_packet<T>(&self, pw: &mut MaplePacketWriter<T>) -> NetResult<()>
    where
        T: BufMut,
    {
        let data = self.as_bytes();
        let padding = N - data.len();
        pw.write_bytes(data);
        for _ in 0..padding {
            pw.write_u8(0);
        }
        Ok(())
    }
}

impl<'de, const N: usize> DecodePacket<'de> for arrayvec::ArrayString<N> {
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let arr = pr.read_array::<N>()?;
        Ok(arrayvec::ArrayString::from_byte_string(&arr)?)
    }
}

impl<const N: usize> PacketLen for arrayvec::ArrayString<N> {
    const SIZE_HINT: Option<usize> = Some(N);

    fn packet_len(&self) -> usize {
        N
    }
}
