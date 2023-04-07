use crate::{DecodePacket, EncodePacket, MaplePacketReader, MaplePacketWriter, NetResult};

pub trait PartialData<'de>: Sized {
    type Flags;
    fn get_flags(&self) -> Self::Flags;
    fn partial_encode_packet<Buf: bytes::BufMut>(
        &self,
        flag: Self::Flags,
        pw: &mut MaplePacketWriter<Buf>,
    ) -> NetResult<()>;
    fn partial_decode_packet(flag: Self::Flags, pr: &mut MaplePacketReader<'de>)
        -> NetResult<Self>;
    fn partial_packet_len(&self, flag: Self::Flags) -> usize;
}

#[derive(Debug, Clone, PartialEq)]
pub struct PartialFlag<Hdr, FlagData> {
    pub hdr: Hdr,
    pub data: FlagData,
}

impl<Hdr, FlagData> PartialFlag<Hdr, FlagData> {
    pub fn new(hdr: Hdr, data: FlagData) -> Self {
        Self { hdr, data }
    }
}

impl<FlagData> From<FlagData> for PartialFlag<(), FlagData> {
    fn from(value: FlagData) -> Self {
        Self::new((), value)
    }
}

impl<'de, Hdr, FlagData> EncodePacket for PartialFlag<Hdr, FlagData>
where
    Hdr: EncodePacket,
    FlagData: PartialData<'de>,
    FlagData::Flags: EncodePacket + std::fmt::Debug,
{
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        let flags = self.data.get_flags();
        flags.packet_len() + self.hdr.packet_len() + self.data.partial_packet_len(flags)
    }

    fn encode_packet<T: bytes::BufMut>(&self, pw: &mut MaplePacketWriter<T>) -> NetResult<()> {
        let flags = self.data.get_flags();
        self.data.get_flags().encode_packet(pw)?;
        self.hdr.encode_packet(pw)?;
        self.data.partial_encode_packet(flags, pw)?;

        Ok(())
    }
}

impl<'de, Hdr, FlagData> DecodePacket<'de> for PartialFlag<Hdr, FlagData>
where
    Hdr: DecodePacket<'de>,
    FlagData: PartialData<'de>,
    FlagData::Flags: DecodePacket<'de>,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let flags = FlagData::Flags::decode_packet(pr)?;
        let hdr = Hdr::decode_packet(pr)?;
        let data = FlagData::partial_decode_packet(flags, pr)?;

        Ok(Self { hdr, data })
    }
}

#[macro_export]
macro_rules! partial_data {
    ($name:ident, $partial_name:ident, $partial_ty:ty, $($stat_name:ident($stat_ty:ty) => $stat_ix:expr),* $(,)?) => {
        bitflags::bitflags! {
            #[derive(Debug, Clone, Default)]
            pub struct $partial_name: $partial_ty {
                $(const $stat_name = $stat_ix;)*
            }
        }

        $crate::mark_maple_bit_flags!($partial_name);

        paste::paste! {
            impl $partial_name {
                $(pub fn [<has_ $stat_name:lower>](&self) -> bool {
                    self.contains(<$partial_name>::$stat_name)
                })*
            }


            #[derive(Debug, Default)]
            pub struct [<$name Partial>] {
                $(
                    pub [<$stat_name:lower>]: $crate::proto::CondOption<$stat_ty>,
                )*
            }

            impl <'de> $crate::proto::partial::PartialData<'de> for [<$name Partial>] {
                type Flags = $partial_name;

                fn get_flags(&self) -> Self::Flags {
                    let mut flags = $partial_name::empty();

                    $(
                        if self.[<$stat_name:lower>].is_some() {
                            flags  |= $partial_name::$stat_name;
                        }
                    )*;

                    flags
                }

                fn partial_encode_packet<Buf: bytes::BufMut>(&self, _flag: Self::Flags, pw: &mut $crate::MaplePacketWriter<Buf>) -> $crate::NetResult<()> {
                    use $crate::EncodePacket;
                    $(
                        self.[<$stat_name:lower>].encode_packet(pw)?;
                    )*
                    Ok(())
                }

                fn partial_decode_packet(flag: Self::Flags, pr: &mut $crate::MaplePacketReader<'de>) -> $crate::NetResult<Self> {
                    use $crate::proto::conditional::{CondOption, MapleConditional};
                    Ok(Self {
                        $([<$stat_name:lower>]: CondOption::<$stat_ty>::decode_packet_cond(
                                flag.contains(<$partial_name>::$stat_name),
                                pr
                            )?
                        ),*
                    })
                }

                fn partial_packet_len(&self, _flag: Self::Flags) -> usize {
                    use $crate::EncodePacket;
                    $(self.[<$stat_name:lower>].packet_len() +)*
                        0
                }
            }

            #[derive(Debug)]
            pub struct [<$name All>] {
                $(pub [<$stat_name:lower>]: $stat_ty,)*
            }

            impl $crate::EncodePacket for [<$name All>] {
                const SIZE_HINT: Option<usize> = None;

                fn packet_len(&self) -> usize {
                    todo!()
                }

                fn encode_packet<T: bytes::BufMut>(&self, pw: &mut $crate::MaplePacketWriter<T>) -> $crate::NetResult<()> {
                    $(self.[<$stat_name:lower>].encode_packet(pw)?;)*
                    Ok(())
                }
            }

            impl<'de> $crate::DecodePacket<'de> for [<$name All>] {
                fn decode_packet(pr: &mut $crate::MaplePacketReader<'de>) -> $crate::NetResult<Self> {
                    Ok(Self{
                        $([<$stat_name:lower>]: <$stat_ty>::decode_packet(pr)?,)*
                    })
                }
            }

            #[derive(Debug, Clone)]
            pub struct [<$partial_name All>];

            impl $crate::proto::PacketWrapped for [<$partial_name All>] {
                type Inner = $partial_ty;

                fn packet_into_inner(&self) -> Self::Inner {
                    <$partial_name>::all().bits()
                }

                fn packet_from(_: Self::Inner) -> Self {
                    Self
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::proto::{
        partial::{PartialData, PartialFlag},
        tests::enc_dec_test,
        CondOption,
    };

    #[test]
    fn test_simple() {
        partial_data!(
            TestStats,
            TestStatsFlags,
            u32,
            A(u8) => 1 << 0,
            B(u16) => 1 << 1,
        );

        impl PartialEq for TestStatsAll {
            fn eq(&self, other: &Self) -> bool {
                self.a == other.a && self.b == other.b
            }
        }

        let _all = TestStatsAll { a: 1, b: 2 };

        let partial = TestStatsPartial {
            a: CondOption(None),
            b: CondOption(None),
        };

        let flags = partial.get_flags();
        assert!(!flags.has_a());
        assert!(!flags.has_b());

        enc_dec_test(TestStatsAll { a: 0xaa, b: 0x1234 });

        impl PartialEq for TestStatsPartial {
            fn eq(&self, other: &Self) -> bool {
                self.a == other.a && self.b == other.b
            }
        }

        pub type TestPartialData = PartialFlag<(), TestStatsPartial>;
        enc_dec_test(TestPartialData::from(TestStatsPartial {
            a: None.into(),
            b: Some(0x1234).into(),
        }));
    }
}
