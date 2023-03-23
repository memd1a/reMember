pub mod analyzer;
pub mod proto;
pub mod reader;
pub mod writer;
pub mod opcode;
pub mod error;

use bytes::{Bytes, BytesMut};
pub use proto::{DecodePacket, DecodePacketOwned, EncodePacket, PacketLen};
pub use proto::conditional::MapleConditional;
pub use reader::MaplePacketReader;
pub use writer::MaplePacketWriter;
pub use opcode::HasOpcode;

pub use error::NetError;
pub type NetResult<T> = Result<T, error::NetError>;

#[derive(Clone, Default, Debug)]
pub struct MaplePacket {
    pub data: Bytes,
}

impl MaplePacket {
    pub fn from_data(data: Bytes) -> Self {
        Self { data }
    }

    pub fn from_writer(pw: MaplePacketWriter<BytesMut>) -> Self {
        Self::from_data(pw.buf.freeze())
    }

    pub fn into_reader(&self) -> MaplePacketReader<'_> {
        MaplePacketReader::new(&self.data)
    }

    pub fn read_opcode(&self) -> NetResult<u16> {
        self.into_reader().read_u16()
    }
}

