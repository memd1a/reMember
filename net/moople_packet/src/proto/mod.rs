pub mod conditional;
pub mod constant;
pub mod list;
pub mod maple_enum;
pub mod option;
pub mod packed_bits;
pub mod padding;
pub mod primitive;
pub mod string;
pub mod time;
pub mod tracing;
pub mod wrapped;

use bytes::BufMut;

pub use list::{MapleIndexList, MapleList16, MapleList32, MapleList64, MapleList8};
pub use packed_bits::MaplePacked;

use crate::{reader::MaplePacketReader, writer::MaplePacketWriter, NetResult};

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
}

pub trait EncodePacket: Sized {
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
}

pub trait DecodePacketOwned: for<'de> DecodePacket<'de> {}
impl<T> DecodePacketOwned for T where T: for<'de> DecodePacket<'de> {}

pub trait PacketLen {
    const SIZE_HINT: Option<usize>;

    fn packet_len(&self) -> usize;
}

/// Helper type to calculate size hint
pub struct SizeHint(pub Option<usize>);

impl SizeHint {
    pub const fn zero() -> Self {
        Self(Some(0))
    }

    /// Sum two Option<usize>
    /// When const traits become stable Add can be implemented
    pub const fn add(self, rhs: Self) -> Self {
        Self(match (self.0, rhs.0) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        })
    }
}

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
            }


            impl<'de, $($name,)*> $crate::proto::DecodePacket<'de> for ($($name,)*)
            where $($name: $crate::proto::DecodePacket<'de>,)* {
                fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
                    Ok((
                        ($($name::decode_packet(pr)?,)*)
                    ))
                }
            }

            impl<$($name,)*> $crate::proto::PacketLen for ($($name,)*)
                where $($name: $crate::proto::PacketLen,)* {
                    const SIZE_HINT: Option<usize> = SizeHint::zero()$(.add(SizeHint($name::SIZE_HINT)))*.0;

                    fn packet_len(&self) -> usize {
                        #[allow(non_snake_case)]
                        let ($($name,)*) = self;

                        $($name.packet_len() +)*0
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
    use super::PacketLen;
    #[test]
    fn tuple_size() {
        assert_eq!(<((), (),)>::SIZE_HINT, Some(0));
        assert_eq!(<((), u32,)>::SIZE_HINT, Some(4));
        assert_eq!(<((), u32, String)>::SIZE_HINT, None);
    }
}
