pub mod analyzer;
pub mod error;
pub mod opcode;
pub mod proto;
pub mod reader;
pub mod util;
pub mod writer;

use bytes::{Bytes, BytesMut};
pub use opcode::HasOpcode;
pub use proto::conditional::MapleConditional;
pub use proto::{DecodePacket, DecodePacketOwned, DecodePacketSized, EncodePacket};
pub use reader::MaplePacketReader;
pub use util::SizeHint;
pub use writer::MaplePacketWriter;

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
