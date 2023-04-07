use bytes::BufMut;

use crate::{MaplePacketReader, MaplePacketWriter, NetResult};

use super::{DecodePacket, EncodePacket};

pub trait PacketWrapped: Sized {
    type Inner;
    fn packet_into_inner(&self) -> Self::Inner;
    fn packet_from(v: Self::Inner) -> Self;
}

pub trait PacketTryWrapped: Sized {
    type Inner;
    fn packet_into_inner(&self) -> Self::Inner;
    fn packet_try_from(v: Self::Inner) -> NetResult<Self>;
}

impl<W> EncodePacket for W
where
    W: PacketTryWrapped,
    W::Inner: EncodePacket,
{
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        self.packet_into_inner().encode_packet(pw)
    }

    const SIZE_HINT: Option<usize> = W::Inner::SIZE_HINT;

    fn packet_len(&self) -> usize {
        Self::SIZE_HINT.unwrap_or(self.packet_into_inner().packet_len())
    }
}

impl<'de, MW> DecodePacket<'de> for MW
where
    MW: PacketTryWrapped,
    MW::Inner: DecodePacket<'de>,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let inner = <MW as PacketTryWrapped>::Inner::decode_packet(pr)?;
        MW::packet_try_from(inner)
    }
}

impl<W: PacketWrapped> PacketTryWrapped for W {
    type Inner = W::Inner;

    fn packet_into_inner(&self) -> Self::Inner {
        self.packet_into_inner()
    }

    fn packet_try_from(v: Self::Inner) -> NetResult<Self> {
        Ok(<W as PacketWrapped>::packet_from(v))
    }
}
