use std::fmt::Debug;

use bytes::BufMut;
use enumflags2::{BitFlag, BitFlags, _internal::RawBitFlags};

use crate::{MaplePacketReader, MaplePacketWriter, NetResult};

use super::{DecodePacket, EncodePacket, PacketLen};

pub trait MapleWrapped: Sized {
    type Inner;
    fn maple_into_inner(&self) -> Self::Inner;
    fn maple_from(v: Self::Inner) -> Self;
}

pub trait MapleTryWrapped: Sized {
    type Inner;
    fn maple_into_inner(&self) -> Self::Inner;
    fn maple_try_from(v: Self::Inner) -> NetResult<Self>;
}

impl<W> EncodePacket for W
where
    W: MapleTryWrapped,
    W::Inner: EncodePacket,
{
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        self.maple_into_inner().encode_packet(pw)
    }
}

impl<'de, MW> DecodePacket<'de> for MW
where
    MW: MapleTryWrapped,
    MW::Inner: DecodePacket<'de>,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let inner = <MW as MapleTryWrapped>::Inner::decode_packet(pr)?;
        MW::maple_try_from(inner)
    }
}

impl<W: MapleWrapped> MapleTryWrapped for W {
    type Inner = W::Inner;

    fn maple_into_inner(&self) -> Self::Inner {
        self.maple_into_inner()
    }

    fn maple_try_from(v: Self::Inner) -> NetResult<Self> {
        Ok(<W as MapleWrapped>::maple_from(v))
    }
}

impl<W> PacketLen for W
where
    W: MapleTryWrapped,
    W::Inner: PacketLen,
{
    const SIZE_HINT: Option<usize> = W::Inner::SIZE_HINT;

    fn packet_len(&self) -> usize {
        self.maple_into_inner().packet_len()
    }
}

impl<T> MapleWrapped for BitFlags<T>
where
    T: BitFlag,
    T: Debug,
{
    type Inner = <T as RawBitFlags>::Numeric;

    fn maple_into_inner(&self) -> Self::Inner {
        self.bits()
    }

    fn maple_from(v: Self::Inner) -> Self {
        Self::from_bits(v).unwrap()
    }
}
