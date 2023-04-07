use bytes::{BufMut, BytesMut};

use crate::{opcode::NetOpcode, MaplePacket};

fn maple128_to_bytes(v: u128) -> [u8; 16] { 
    let mut raw_flags: [u32; 4] = bytemuck::cast(v.to_le_bytes());
    raw_flags.reverse();
    bytemuck::cast(raw_flags)
}

#[derive(Debug)]
pub struct MaplePacketWriter<T = BytesMut> {
    pub buf: T,
}

impl Default for MaplePacketWriter<BytesMut> {
    fn default() -> Self {
        Self {
            buf: Default::default(),
        }
    }
}

impl<T> MaplePacketWriter<T> {
    pub fn into_inner(self) -> T {
        self.buf
    }
}

impl MaplePacketWriter<BytesMut> {
    pub fn with_capacity(cap: usize) -> Self {
        Self::new(BytesMut::with_capacity(cap))
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn into_packet(self) -> MaplePacket {
        MaplePacket::from_writer(self)
    }
}

impl<T> MaplePacketWriter<T>
where
    T: BufMut,
{
    pub fn new(buf: T) -> Self {
        Self { buf }
    }

    pub fn write_opcode(&mut self, op: impl NetOpcode) {
        self.write_u16(op.into())
    }

    pub fn write_u8(&mut self, v: u8) {
        self.buf.put_u8(v);
    }

    pub fn write_i8(&mut self, v: i8) {
        self.buf.put_i8(v);
    }

    pub fn write_bool(&mut self, v: bool) {
        self.write_u8(v.into())
    }

    pub fn write_i16(&mut self, v: i16) {
        self.buf.put_i16_le(v);
    }

    pub fn write_i32(&mut self, v: i32) {
        self.buf.put_i32_le(v);
    }

    pub fn write_i64(&mut self, v: i64) {
        self.buf.put_i64_le(v);
    }

    pub fn write_i128(&mut self, v: i128) {
        self.write_u128(v as u128);
    }

    pub fn write_f32(&mut self, v: f32) {
        self.buf.put_f32_le(v);
    }

    pub fn write_f64(&mut self, v: f64) {
        self.buf.put_f64_le(v);
    }

    pub fn write_u16(&mut self, v: u16) {
        self.buf.put_u16_le(v);
    }

    pub fn write_u32(&mut self, v: u32) {
        self.buf.put_u32_le(v);
    }

    pub fn write_u64(&mut self, v: u64) {
        self.buf.put_u64_le(v);
    }

    pub fn write_u128(&mut self, v: u128) {
        self.write_array::<16>(&maple128_to_bytes(v));
    }

    pub fn write_bytes(&mut self, v: &[u8]) {
        self.buf.put(v);
    }

    pub fn write_array<const N: usize>(&mut self, v: &[u8; N]) {
        self.buf.put(v.as_slice());
    }

    pub fn write_str(&mut self, v: &str) {
        let b = v.as_bytes();
        self.write_u16(b.len() as u16);
        self.write_bytes(b);
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.buf
    }

    pub fn get_ref(&mut self) -> &T {
        &self.buf
    }
}

#[cfg(test)]
mod tests {
    use super::MaplePacketWriter;

    #[test]
    fn write() {
        let mut pw = MaplePacketWriter::with_capacity(64);
        pw.write_u8(0);
        pw.write_bytes(&[1, 2, 3, 4]);

        assert_eq!(pw.len(), 5);
    }
}
