use moople_packet::{
    DecodePacket, EncodePacket, MaplePacketReader, MaplePacketWriter, NetResult, PacketLen,
};

//TODO: move this to moople_packet and make it even more generic

pub trait PartialFlagData<'de>: Sized {
    type Flags;
    fn get_flags(&self) -> Self::Flags;
    fn flag_encode_packet<Buf: bytes::BufMut>(
        &self,
        flag: Self::Flags,
        pw: &mut MaplePacketWriter<Buf>,
    ) -> NetResult<()>;
    fn flag_decode_packet(flag: Self::Flags, pr: &mut MaplePacketReader<'de>) -> NetResult<Self>;
    fn flag_packet_len(&self, flag: Self::Flags) -> usize;
}

#[derive(Debug, Clone)]
pub struct PartialFlag<Hdr, FlagData> {
    pub hdr: Hdr,
    pub data: FlagData,
}

impl<'de, Hdr, FlagData> EncodePacket for PartialFlag<Hdr, FlagData>
where
    Hdr: EncodePacket,
    FlagData: PartialFlagData<'de>,
    FlagData::Flags: EncodePacket,
{
    fn encode_packet<T: bytes::BufMut>(&self, pw: &mut MaplePacketWriter<T>) -> NetResult<()> {
        let flags = self.data.get_flags();
        self.data.get_flags().encode_packet(pw)?;
        self.hdr.encode_packet(pw)?;
        self.data.flag_encode_packet(flags, pw)?;


        Ok(())
    }
}

impl<'de, Hdr, FlagData> DecodePacket<'de> for PartialFlag<Hdr, FlagData>
where
    Hdr: DecodePacket<'de>,
    FlagData: PartialFlagData<'de>,
    FlagData::Flags: DecodePacket<'de>,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let flags = FlagData::Flags::decode_packet(pr)?;
        let hdr = Hdr::decode_packet(pr)?;
        let data = FlagData::flag_decode_packet(flags, pr)?;

        Ok(Self { hdr, data })
    }
}

impl<'de, Hdr, FlagData> PacketLen for PartialFlag<Hdr, FlagData>
where
    Hdr: PacketLen,
    FlagData: PartialFlagData<'de>,
    FlagData::Flags: PacketLen,
{
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        let flags = self.data.get_flags();
        flags.packet_len() + self.hdr.packet_len() + self.data.flag_packet_len(flags)
    }
}

#[macro_export]
macro_rules! maple_stats {
    ($name:ident, $flag_name:ident, $flag_ty:ty, $hdr_ty:ty, $($stat_name:ident($stat_ty:ty) => $stat_ix:expr),* $(,)?) => {
        bitflags::bitflags! {
            #[derive(Debug, Clone, Default)]
            pub struct $flag_name: $flag_ty {
                $(const $stat_name = 1 << $stat_ix;)*
            }
        }

        moople_packet::mark_maple_bit_flags!($flag_name);

        paste::paste! {
            impl $flag_name {
                $(pub fn [<has_ $stat_name:lower>](&self) -> bool {
                    self.contains(<$flag_name>::$stat_name)
                })*
            }


            #[derive(Debug)]
            pub struct [<$name Partial>] {
                $(
                    pub [<$stat_name:lower>]: moople_packet::proto::CondOption<$stat_ty>,
                )*
            }

            impl <'de> $crate::stats::PartialFlagData<'de> for [<$name Partial>] {
                type Flags = $flag_name;

                fn get_flags(&self) -> Self::Flags {
                    let mut flags = $flag_name::empty();

                    $(
                        if self.[<$stat_name:lower>].is_some() {
                            flags = flags |  $flag_name::$stat_name;
                        }
                    )*;

                    flags
                }

                fn flag_encode_packet<Buf: bytes::BufMut>(&self, _flag: Self::Flags, pw: &mut moople_packet::MaplePacketWriter<Buf>) -> moople_packet::NetResult<()> {
                    use moople_packet::EncodePacket;
                    $(
                        self.[<$stat_name:lower>].encode_packet(pw)?;
                    )*
                    Ok(())
                }

                fn flag_decode_packet(flag: Self::Flags, pr: &mut moople_packet::MaplePacketReader<'de>) -> moople_packet::NetResult<Self> {
                    use moople_packet::proto::conditional::{CondOption, MapleConditional};
                    Ok(Self {
                        $([<$stat_name:lower>]: CondOption::<$stat_ty>::decode_packet_cond(
                                flag.contains(<$flag_name>::$stat_name),
                                pr
                            )?
                        ),*
                    })
                }

                fn flag_packet_len(&self, _flag: Self::Flags) -> usize {
                    use moople_packet::PacketLen;
                    $(self.[<$stat_name:lower>].packet_len() +)*
                        0
                }
            }

            #[derive(Debug, moople_derive::MooplePacket)]
            pub struct [<$name All>] {
                $(pub [<$stat_name:lower>]: $stat_ty,)*
            }

            pub struct [<$flag_name All>];

            impl moople_packet::proto::MapleWrapped for [<$flag_name All>] {
                type Inner = $flag_ty;

                fn maple_into_inner(&self) -> Self::Inner {
                    <$flag_name>::all().bits()
                }

                fn maple_from(_: Self::Inner) -> Self {
                    Self
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use moople_packet::proto::CondOption;

    use crate::stats::PartialFlagData;

    #[test]
    fn test_simple() {
        maple_stats!(
            TestStats,
            TestStatsFlags,
            u32,
            (),
            A(u8) => 0,
            B(u16) => 1,
        );

        let _all = TestStatsAll {
            a: 1,
            b: 2,
        };

        let partial = TestStatsPartial {
            a: CondOption(None),
            b: CondOption(None),
        };

        let flags = partial.get_flags();

        assert!(!flags.has_a());
        assert!(!flags.has_b());
    }
}
