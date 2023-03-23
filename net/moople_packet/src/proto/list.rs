use std::fmt::Debug;
use std::marker::PhantomData;

use bytes::BufMut;

use crate::{reader::MaplePacketReader, MaplePacketWriter, NetResult};

use super::{DecodePacket, DecodePacketOwned, EncodePacket, PacketLen};

pub trait MapleListLen: EncodePacket + DecodePacketOwned {
    fn to_len(&self) -> usize;
    fn from_len(ix: usize) -> Self;
}

pub trait MapleListIndex: MapleListLen + PartialEq {
    const TERMINATOR: Self;
}

pub trait MapleListIndexZ: MapleListLen + PartialEq {
    const TERMINATOR: Self;
}

macro_rules! impl_list_index {
    ($ty:ty) => {
        impl MapleListLen for $ty {
            fn to_len(&self) -> usize {
                *self as usize
            }

            fn from_len(ix: usize) -> Self {
                ix as $ty
            }
        }

        impl MapleListIndex for $ty {
            const TERMINATOR: Self = <$ty>::MAX;
        }

        impl MapleListIndexZ for $ty {
            const TERMINATOR: Self = <$ty>::MIN;
        }
    };
}

impl_list_index!(u8);
impl_list_index!(u16);
impl_list_index!(u32);
impl_list_index!(u64);

#[derive(Debug, Clone)]
pub struct MapleIndexList<I, T> {
    pub items: Vec<(I, T)>,
}

impl<I, T> From<Vec<(I, T)>> for MapleIndexList<I, T> {
    fn from(items: Vec<(I, T)>) -> Self {
        Self { items }
    }
}

impl<I, T> Default for MapleIndexList<I, T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<'de, I, T> DecodePacket<'de> for MapleIndexList<I, T>
where
    T: DecodePacket<'de>,
    I: MapleListIndex,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let mut items = Vec::new();

        loop {
            let ix = I::decode_packet(pr)?;
            if ix == I::TERMINATOR {
                break;
            }
            let item = T::decode_packet(&mut *pr)?;
            items.push((ix, item));
        }

        Ok(MapleIndexList { items })
    }
}

impl<I, T> EncodePacket for MapleIndexList<I, T>
where
    T: EncodePacket,
    I: MapleListIndex,
{
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        let items = &self.items;

        for (ix, item) in items.iter() {
            ix.encode_packet(pw)?;
            item.encode_packet(pw)?;
        }
        I::TERMINATOR.encode_packet(pw)?;

        Ok(())
    }
}

impl<I, T> PacketLen for MapleIndexList<I, T>
where
    T: PacketLen,
    I: PacketLen,
{
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        I::SIZE_HINT.unwrap() + self.items.iter().map(|v| v.packet_len()).sum::<usize>()
    }
}

/// Like `MapleIndexList`just using zero index as terminator
#[derive(Debug, Clone)]
pub struct MapleIndexListZ<I, T> {
    pub items: Vec<(I, T)>,
}

impl<I, E> FromIterator<(I, E)> for MapleIndexListZ<I, E> {
    fn from_iter<T: IntoIterator<Item = (I, E)>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect()
        }
    }
}

impl<I, T> Default for MapleIndexListZ<I, T> {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl<I, T> From<Vec<(I, T)>> for MapleIndexListZ<I, T> {
    fn from(items: Vec<(I, T)>) -> Self {
        Self { items }
    }
}

impl<'de, I, T> DecodePacket<'de> for MapleIndexListZ<I, T>
where
    T: DecodePacket<'de>,
    I: MapleListIndexZ,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let mut items = Vec::new();

        loop {
            let ix = I::decode_packet(pr)?;
            if ix == I::TERMINATOR {
                break;
            }
            let item = T::decode_packet(&mut *pr)?;
            items.push((ix, item));
        }

        Ok(Self { items })
    }
}

impl<I, T> EncodePacket for MapleIndexListZ<I, T>
where
    T: EncodePacket,
    I: MapleListIndexZ,
{
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        let items = &self.items;

        for (ix, item) in items.iter() {
            ix.encode_packet(pw)?;
            item.encode_packet(pw)?;
        }
        I::TERMINATOR.encode_packet(pw)?;

        Ok(())
    }
}

impl<I, T> PacketLen for MapleIndexListZ<I, T>
where
    T: PacketLen,
    I: MapleListIndex + PacketLen,
{
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        I::SIZE_HINT.unwrap() + self.items.iter().map(|v| v.packet_len()).sum::<usize>()
    }
}

#[derive(Clone)]
pub struct MapleList<I, T> {
    pub items: Vec<T>,
    pub _index: PhantomData<I>,
}


impl<I, E> FromIterator<E> for MapleList<I, E> {
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
            _index: PhantomData,
        }
    }
}

impl<I, T> Default for MapleList<I, T> {
    fn default() -> Self {
        Self {
            items: Vec::default(),
            _index: PhantomData,
        }
    }
}

impl<I, T> From<Vec<T>> for MapleList<I, T> {
    fn from(items: Vec<T>) -> Self {
        Self {
            items,
            _index: PhantomData,
        }
    }
}

impl<I, T> Debug for MapleList<I, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapleList")
            .field("items", &self.items)
            .finish()
    }
}

impl<'de, I, T> DecodePacket<'de> for MapleList<I, T>
where
    I: MapleListLen,
    T: DecodePacket<'de>,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let n = I::decode_packet(pr)?;
        let n = n.to_len();

        Ok(Self {
            items: T::decode_packet_n(pr, n)?,
            _index: PhantomData,
        })
    }
}

impl<I, T> EncodePacket for MapleList<I, T>
where
    I: MapleListLen,
    T: EncodePacket,
{
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        let items = &self.items;
        I::from_len(items.len()).encode_packet(pw)?;
        T::encode_packet_n(items, pw)?;

        Ok(())
    }
}

impl<I, T> PacketLen for MapleList<I, T>
where
    T: PacketLen,
    I: PacketLen,
{
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        I::SIZE_HINT.unwrap() + self.items.iter().map(|v| v.packet_len()).sum::<usize>()
    }
}

pub type MapleList8<T> = MapleList<u8, T>;
pub type MapleList16<T> = MapleList<u16, T>;
pub type MapleList32<T> = MapleList<u32, T>;
pub type MapleList64<T> = MapleList<u64, T>;

pub type MapleIndexList8<T> = MapleIndexList<u8, T>;
pub type MapleIndexList16<T> = MapleIndexList<u16, T>;
pub type MapleIndexList32<T> = MapleIndexList<u32, T>;
pub type MapleIndexList64<T> = MapleIndexList<u64, T>;

pub type MapleIndexListZ8<T> = MapleIndexListZ<u8, T>;
pub type MapleIndexListZ16<T> = MapleIndexListZ<u16, T>;
pub type MapleIndexListZ32<T> = MapleIndexListZ<u32, T>;
pub type MapleIndexListZ64<T> = MapleIndexListZ<u64, T>;
