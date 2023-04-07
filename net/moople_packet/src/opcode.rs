use crate::{error::NetError, DecodePacket, EncodePacket, NetResult};

pub trait NetOpcode: TryFrom<u16> + Into<u16> + Copy + Clone + Send + Sync {
    fn get_opcode(v: u16) -> NetResult<Self> {
        Self::try_from(v).map_err(|_| NetError::InvalidOpcode(v))
    }
}

impl NetOpcode for u16 {}

pub trait HasOpcode {
    type OP: NetOpcode;

    const OPCODE: Self::OP;
}

#[derive(Debug, Default)]
pub struct WithOpcode<const OP: u16, T>(pub T);
impl<const OP: u16, T> HasOpcode for WithOpcode<OP, T> {
    type OP = u16;

    const OPCODE: Self::OP = OP;
}

impl<const OP: u16, T> EncodePacket for WithOpcode<OP, T>
where
    T: EncodePacket,
{
    const SIZE_HINT: Option<usize> = T::SIZE_HINT;

    fn packet_len(&self) -> usize {
        self.0.packet_len()
    }

    fn encode_packet<B: bytes::BufMut>(
        &self,
        pw: &mut crate::MaplePacketWriter<B>,
    ) -> NetResult<()> {
        self.0.encode_packet(pw)
    }
}

impl<'de, const OP: u16, T> DecodePacket<'de> for WithOpcode<OP, T>
where
    T: DecodePacket<'de>,
{
    fn decode_packet(pr: &mut crate::MaplePacketReader<'de>) -> NetResult<Self> {
        Ok(Self(T::decode_packet(pr)?))
    }
}

#[macro_export]
macro_rules! packet_opcode {
    ($packet_ty:ty, $op:path, $ty:ty) => {
        impl $crate::HasOpcode for $packet_ty {
            type OP = $ty;

            const OPCODE: Self::OP = $op;
        }
    };
    ($packet_ty:ty, $ty:ident::$op:ident) => {
        impl $crate::HasOpcode for $packet_ty {
            type OP = $ty;

            const OPCODE: Self::OP = $ty::$op;
        }
    };
}
