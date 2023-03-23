use std::iter;

use bytes::BytesMut;
use itertools::Itertools;
use moople_packet::{EncodePacket, HasOpcode, MaplePacketWriter, NetResult};

/// Buffer to allow to encode multiple packets onto one buffer
/// while still allowing to iterate over the encoded packets
#[derive(Debug)]
pub struct PacketBuffer {
    buf: BytesMut,
    ix: Vec<usize>,
}

impl Default for PacketBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketBuffer {
    pub fn new() -> Self {
        Self {
            buf: BytesMut::new(),
            ix: Vec::new(),
        }
    }

    /// Encode a packet onto the buffer
    pub fn write_packet<T: EncodePacket + HasOpcode>(&mut self, pkt: T) -> NetResult<()> {
        let ix = self.buf.len();
        let mut pw = MaplePacketWriter::new(&mut self.buf);

        pw.write_opcode(T::OPCODE);
        if let Err(err) = pkt.encode_packet(&mut pw) {
            self.buf.truncate(ix);
            return Err(err);
        }

        self.ix.push(ix);
        Ok(())
    }

    /// Iterator over the written packet frames
    pub fn packets(&self) -> impl Iterator<Item = &[u8]> + '_ {
        self.ix
            .iter()
            .cloned()
            .chain(iter::once(self.buf.len()))
            .tuple_windows()
            .map(|(l, r)| &self.buf[l..r])
    }

    /// Clears the buffer
    pub fn clear(&mut self) {
        self.buf.truncate(0);
        self.ix.clear();
    }
}

#[cfg(test)]
mod tests {
    use moople_packet::opcode::WithOpcode;

    use super::PacketBuffer;

    #[test]
    fn packet_buf() -> anyhow::Result<()> {
        let mut buf = PacketBuffer::new();
        buf.write_packet(WithOpcode::<1, u8>(1))?;
        buf.write_packet(WithOpcode::<1, u8>(2))?;
        buf.write_packet(WithOpcode::<1, u8>(3))?;

        itertools::assert_equal(buf.packets(), [[1, 0, 1], [1, 0, 2], [1, 0, 3]]);

        buf.clear();

        assert_eq!(buf.packets().count(), 0);
        buf.write_packet(WithOpcode::<1, u8>(1))?;
        itertools::assert_equal(buf.packets(), [[1, 0, 1]]);

        Ok(())
    }
}
