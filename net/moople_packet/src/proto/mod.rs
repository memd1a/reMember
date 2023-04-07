pub mod partial;
pub mod bits;
pub mod conditional;
pub mod constant;
pub mod geo;
pub mod list;
pub mod maple_enum;
pub mod option;
pub mod padding;
pub mod primitive;
pub mod string;
pub mod time;
pub mod tracing;
pub mod wrapped;

use bytes::BufMut;

use crate::{reader::MaplePacketReader, writer::MaplePacketWriter, MaplePacket, NetResult};
pub use conditional::{CondEither, CondOption};
pub use list::{
    MapleIndexList, MapleList, MapleList16, MapleList32, MapleList64, MapleList8, MapleListIndexZ,
};
pub use wrapped::{PacketTryWrapped, PacketWrapped};

pub trait DecodePacket<'de>: Sized {
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self>;

    fn decode_packet_n(pr: &mut MaplePacketReader<'de>, n: usize) -> NetResult<Vec<Self>> {
        (0..n)
            .map(|_| Self::decode_packet(pr))
            .collect::<NetResult<_>>()
    }

    /// Attempts to decode the packet
    /// If EOF is reached None is returned elsewise the Error is returned
    /// This is useful for reading an optional tail
    fn try_decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Option<Self>> {
        let mut sub_reader = pr.sub_reader();
        Ok(match Self::decode_packet(&mut sub_reader) {
            Ok(item) => {
                pr.commit_sub_reader(sub_reader)?;
                Some(item)
            }
            Err(crate::NetError::EOF { .. }) => None,
            Err(err) => return Err(err),
        })
    }

    fn decode_from_data(data: &'de [u8]) -> NetResult<Self> {
        let mut r = MaplePacketReader::new(data);
        Self::decode_packet(&mut r)
    }

    fn decode_from_data_complete(data: &'de [u8]) -> anyhow::Result<Self> {
        let mut r = MaplePacketReader::new(data);
        let res = Self::decode_packet(&mut r)?;
        if !r.remaining_slice().is_empty() {
            anyhow::bail!("Still remaining data: {:?}", r.remaining_slice());
        }
        Ok(res)
    }
}

pub trait EncodePacket: Sized {
    const SIZE_HINT: Option<usize>;

    fn packet_len(&self) -> usize;

    fn encode_packet<T: BufMut>(&self, pw: &mut MaplePacketWriter<T>) -> NetResult<()>;

    /// Encodes this data as slice
    fn encode_packet_n<T: BufMut>(items: &[Self], pw: &mut MaplePacketWriter<T>) -> NetResult<()> {
        for item in items.iter() {
            item.encode_packet(pw)?;
        }

        Ok(())
    }

    fn to_data(&self) -> NetResult<bytes::Bytes> {
        let mut pw = MaplePacketWriter::default();
        self.encode_packet(&mut pw)?;
        Ok(pw.into_inner().freeze())
    }

    fn to_packet(&self) -> NetResult<MaplePacket> {
        Ok(MaplePacket::from_data(self.to_data()?))
    }
}

pub trait DecodePacketSized<'de, T>: Sized {
    fn decode_packet_sized(pr: &mut MaplePacketReader<'de>, size: usize) -> NetResult<Self>;
}

impl<'de, T> DecodePacketSized<'de, T> for Vec<T> where T: DecodePacket<'de> {
    fn decode_packet_sized(pr: &mut MaplePacketReader<'de>, size: usize) -> NetResult<Self> {
        T::decode_packet_n(pr, size)
    }
}

pub trait DecodePacketOwned: for<'de> DecodePacket<'de> {}
impl<T> DecodePacketOwned for T where T: for<'de> DecodePacket<'de> {}


macro_rules! impl_packet {
    ( $($name: ident)* ) => {
        impl<$($name,)*> $crate::proto::EncodePacket for ($($name,)*)
            where $($name: $crate::proto::EncodePacket,)* {
                fn encode_packet<T: BufMut>(&self, pw: &mut MaplePacketWriter<T>) -> NetResult<()> {
                    #[allow(non_snake_case)]
                    let ($($name,)*) = self;
                    $($name.encode_packet(pw)?;)*
                    Ok(())
                }

                const SIZE_HINT: Option<usize> = $crate::util::SizeHint::zero()
                        $(.add($crate::util::SizeHint($name::SIZE_HINT)))*.0;

                fn packet_len(&self) -> usize {
                    #[allow(non_snake_case)]
                    let ($($name,)*) = self;

                    $($name.packet_len() +)*0
                }
            }


            impl<'de, $($name,)*> $crate::proto::DecodePacket<'de> for ($($name,)*)
            where $($name: $crate::proto::DecodePacket<'de>,)* {
                fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
                    Ok((
                        ($($name::decode_packet(pr)?,)*)
                    ))
                }
            }
    }
}

macro_rules! impl_for_tuples {
    ($apply_macro:ident) => {
        $apply_macro! { A }
        $apply_macro! { A B }
        $apply_macro! { A B C }
        $apply_macro! { A B C D }
        $apply_macro! { A B C D E }
        $apply_macro! { A B C D E F }
        $apply_macro! { A B C D E F G }
        $apply_macro! { A B C D E F G H }
        $apply_macro! { A B C D E F G H I }
        $apply_macro! { A B C D E F G H I J }
        $apply_macro! { A B C D E F G H I J K }
        $apply_macro! { A B C D E F G H I J K L }
    };
}

impl_for_tuples!(impl_packet);

#[cfg(test)]
mod tests {
    use crate::{DecodePacketOwned, EncodePacket};

    /// Helper function to test If encoding matches decoding
    pub(crate) fn enc_dec_test<T>(val: T)
    where
        T: EncodePacket + DecodePacketOwned + PartialEq + std::fmt::Debug,
    {
        let data = val.to_packet().expect("encode");
        let mut pr = data.into_reader();
        let decoded = T::decode_packet(&mut pr).expect("decode");

        assert_eq!(val, decoded);
    }

    /// Helper function to test If encoding matches decoding
    pub(crate) fn enc_dec_test_all<T>(vals: impl IntoIterator<Item = T>)
    where
        T: EncodePacket + DecodePacketOwned + PartialEq + std::fmt::Debug,
    {
        for val in vals {
            enc_dec_test(val);
        }
    }

    #[test]
    fn tuple_size() {
        assert_eq!(<((), (),)>::SIZE_HINT, Some(0));
        assert_eq!(<((), u32,)>::SIZE_HINT, Some(4));
        assert_eq!(<((), u32, String)>::SIZE_HINT, None);
    }
}
