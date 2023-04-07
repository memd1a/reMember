use std::ops::{Deref, DerefMut};

use bytes::BufMut;
use either::Either;

use crate::{reader::MaplePacketReader, writer::MaplePacketWriter, NetResult};

use super::{DecodePacket, EncodePacket};

pub trait MapleConditional<'de>: Sized {
    fn encode_packet_cond<B: BufMut>(
        &self,
        cond: bool,
        pw: &mut MaplePacketWriter<B>,
    ) -> NetResult<()>;
    fn decode_packet_cond(cond: bool, pr: &mut MaplePacketReader<'de>) -> NetResult<Self>;
    fn packet_len_cond(&self, cond: bool) -> usize;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CondOption<T>(pub Option<T>);

impl<T> Default for CondOption<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> From<Option<T>> for CondOption<T> {
    fn from(value: Option<T>) -> Self {
        CondOption(value)
    }
}

impl<T> From<CondOption<T>> for Option<T> {
    fn from(value: CondOption<T>) -> Self {
        value.0
    }
}

impl<T: EncodePacket> EncodePacket for CondOption<T> {
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        self.0
            .as_ref()
            .map(|p| p.encode_packet(pw))
            .unwrap_or(Ok(()))
    }

    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        self.0.as_ref().map(|v| v.packet_len()).unwrap_or(0)
    }
}

impl<'de, T> MapleConditional<'de> for CondOption<T>
where
    T: EncodePacket + DecodePacket<'de>,
{
    fn encode_packet_cond<B: BufMut>(
        &self,
        cond: bool,
        pw: &mut MaplePacketWriter<B>,
    ) -> NetResult<()> {
        if cond {
            self.as_ref().expect("Must have value").encode_packet(pw)?;
        }
        Ok(())
    }

    fn decode_packet_cond(cond: bool, pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        Ok(Self(if cond {
            Some(T::decode_packet(pr)?)
        } else {
            None
        }))
    }

    fn packet_len_cond(&self, cond: bool) -> usize {
        cond.then(|| self.as_ref().expect("Must have value").packet_len())
            .unwrap_or(0)
    }
}

impl<T> Deref for CondOption<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CondOption<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CondEither<L, R>(pub Either<L, R>);

impl<L, R> From<Either<L, R>> for CondEither<L, R> {
    fn from(value: Either<L, R>) -> Self {
        CondEither(value)
    }
}
impl<'de, L, R> MapleConditional<'de> for CondEither<L, R>
where
    L: EncodePacket + DecodePacket<'de>,
    R: EncodePacket + DecodePacket<'de>,
{
    fn encode_packet_cond<B: BufMut>(
        &self,
        cond: bool,
        pw: &mut MaplePacketWriter<B>,
    ) -> NetResult<()> {
        if cond {
            self.0
                .as_ref()
                .left()
                .expect("must have value")
                .encode_packet(pw)?;
        } else {
            self.0
                .as_ref()
                .right()
                .expect("must have value")
                .encode_packet(pw)?;
        }

        Ok(())
    }

    fn decode_packet_cond(cond: bool, pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        Ok(Self(if cond {
            Either::Left(L::decode_packet(pr)?)
        } else {
            Either::Right(R::decode_packet(pr)?)
        }))
    }

    fn packet_len_cond(&self, _cond: bool) -> usize {
        //TODO use cond?
        match &self.0 {
            Either::Left(v) => v.packet_len(),
            Either::Right(v) => v.packet_len(),
        }
    }
}
