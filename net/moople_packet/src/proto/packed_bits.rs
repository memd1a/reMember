use bytes::BufMut;
use packed_struct::PackedStruct;

use crate::{MaplePacketReader, MaplePacketWriter, NetResult};

use super::{wrapped::MapleWrapped, DecodePacket, EncodePacket, PacketLen};

pub trait MaplePackedMarker: PackedStruct {}

impl<M: MaplePackedMarker> MapleWrapped for M {
    type Inner = M::ByteArray;

    fn maple_into_inner(&self) -> Self::Inner {
        self.pack().unwrap()
    }

    fn maple_from(v: Self::Inner) -> Self {
        M::unpack(&v).unwrap()
    }
}

#[macro_export]
macro_rules! maple_packed {
    ($ty:ty) => {
        impl MapleWrapped for $ty {
            type Inner = <$ty>::ByteArray;

            fn maple_into_inner(&self) -> Self::Inner {
                self.pack().unwrap()
            }

            fn maple_try_from(v: Self::Inner) -> NetResult<Self> {}
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct MaplePacked<T>(pub T);

impl<const N: usize, T> PacketLen for MaplePacked<T>
where
    T: PackedStruct<ByteArray = [u8; N]>,
{
    const SIZE_HINT: Option<usize> = Some(N);

    fn packet_len(&self) -> usize {
        N
    }
}

impl<const N: usize, T> EncodePacket for MaplePacked<T>
where
    T: PackedStruct<ByteArray = [u8; N]>,
{
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        pw.write_array(&self.0.pack().unwrap());
        Ok(())
    }
}

impl<'de, const N: usize, T> DecodePacket<'de> for MaplePacked<T>
where
    T: PackedStruct<ByteArray = [u8; N]>,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let arr: T::ByteArray = pr.read_array()?;
        Ok(MaplePacked(T::unpack(&arr).unwrap()))
    }
}
