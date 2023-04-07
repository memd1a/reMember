use std::marker::PhantomData;

use crate::{MaplePacketReader, MaplePacketWriter, NetResult};

use super::{wrapped::PacketWrapped, DecodePacket, DecodePacketOwned, EncodePacket};

pub trait MapleOptionIndex: EncodePacket + DecodePacketOwned {
    const NONE_VALUE: Self;
    const SOME_VALUE: Self;
    fn has_value(&self) -> bool;
}

impl MapleOptionIndex for u8 {
    const NONE_VALUE: Self = 0;
    const SOME_VALUE: Self = 1;
    fn has_value(&self) -> bool {
        *self != 0
    }
}

impl MapleOptionIndex for bool {
    const NONE_VALUE: Self = false;
    const SOME_VALUE: Self = true;
    fn has_value(&self) -> bool {
        *self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevMapleOptionIndex<Opt>(pub Opt);

impl<Opt> PacketWrapped for RevMapleOptionIndex<Opt>
where
    Opt: Copy,
{
    type Inner = Opt;

    fn packet_into_inner(&self) -> Self::Inner {
        self.0
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self(v)
    }
}

impl<Opt> MapleOptionIndex for RevMapleOptionIndex<Opt>
where
    Opt: MapleOptionIndex + Copy,
{
    const NONE_VALUE: Self = RevMapleOptionIndex(Opt::SOME_VALUE);
    const SOME_VALUE: Self = RevMapleOptionIndex(Opt::NONE_VALUE);

    fn has_value(&self) -> bool {
        !self.0.has_value()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapleOption<T, Opt> {
    pub opt: Option<T>,
    _t: PhantomData<Opt>,
}

impl<T, Opt> MapleOption<T, Opt> {
    pub fn from_opt(opt: Option<T>) -> Self {
        Self {
            opt,
            _t: PhantomData,
        }
    }
}

impl<T, Opt> From<MapleOption<T, Opt>> for Option<T> {
    fn from(val: MapleOption<T, Opt>) -> Self {
        val.opt
    }
}

impl<T, Opt> From<Option<T>> for MapleOption<T, Opt> {
    fn from(value: Option<T>) -> Self {
        Self::from_opt(value)
    }
}

impl<T, Opt> EncodePacket for MapleOption<T, Opt>
where
    T: EncodePacket,
    Opt: MapleOptionIndex,
{
    fn encode_packet<B: bytes::BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        match self.opt.as_ref() {
            Some(v) => {
                Opt::SOME_VALUE.encode_packet(pw)?;
                v.encode_packet(pw)
            }
            None => Opt::NONE_VALUE.encode_packet(pw),
        }
    }

    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        match self.opt.as_ref() {
            Some(v) => Opt::SOME_VALUE.packet_len() + v.packet_len(),
            None => Opt::NONE_VALUE.packet_len(),
        }
    }
}

impl<'de, T, Opt> DecodePacket<'de> for MapleOption<T, Opt>
where
    T: DecodePacket<'de>,
    Opt: MapleOptionIndex,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let d = Opt::decode_packet(pr)?;
        let v = if d.has_value() {
            Some(T::decode_packet(pr)?)
        } else {
            None
        };

        Ok(Self::from_opt(v))
    }
}

pub type MapleOption8<T> = MapleOption<T, u8>;
pub type MapleOptionR8<T> = MapleOption<T, RevMapleOptionIndex<u8>>;
pub type MapleOptionBool<T> = MapleOption<T, bool>;
pub type MapleOptionRBool<T> = MapleOption<T, RevMapleOptionIndex<bool>>;


#[cfg(test)]
mod tests {
    use crate::proto::tests::enc_dec_test_all;

    use super::*;

    #[test]
    fn option() {
        enc_dec_test_all([
            MapleOption8::from_opt(Some("abc".to_string())),
            MapleOption8::from_opt(None),
        ]);
        enc_dec_test_all([
            MapleOptionR8::from_opt(Some("abc".to_string())),
            MapleOptionR8::from_opt(None),
        ]);
        enc_dec_test_all([
            MapleOptionBool::from_opt(Some("abc".to_string())),
            MapleOptionBool::from_opt(None),
        ]);
        enc_dec_test_all([
            MapleOptionRBool::from_opt(Some("abc".to_string())),
            MapleOptionRBool::from_opt(None),
        ]);
    }
}