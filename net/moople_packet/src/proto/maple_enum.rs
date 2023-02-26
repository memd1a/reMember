#[macro_export]
macro_rules! maple_enum_code {
    ($name:ident, $repr_ty:ty, $($code_name:ident = $val:expr),+) => {
        #[derive(Debug, Clone, PartialEq, Eq, num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
        #[repr($repr_ty)]
        pub enum $name {
            $($code_name = $val,)*
        }

        $crate::mark_maple_enum!($name);
    };
}

#[macro_export]
macro_rules! mark_maple_enum {
    ($enum_ty:ty) => {
        impl $crate::proto::wrapped::MapleTryWrapped for $enum_ty {
            type Inner = <$enum_ty as num_enum::TryFromPrimitive>::Primitive;

            fn maple_try_from(v: Self::Inner) -> $crate::NetResult<Self> {
                use num_enum::TryFromPrimitive;
                Ok(<$enum_ty>::try_from_primitive(v)?)
            }
            fn maple_into_inner(&self) -> Self::Inner {
                Self::Inner::from(self.clone())
            }
        }
    };
}

//TODO support docs
#[macro_export]
macro_rules! maple_packet_enum {
    ($name:ident, $ix_ty:ty, $($variant_name:ident($variant_ty:ty) => $variant_ix:expr),* $(,)?) => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $variant_name($variant_ty)
            ),*
        }

        impl $crate::EncodePacket for $name {
            fn encode_packet<B: bytes::BufMut>(&self, pw: &mut $crate::MaplePacketWriter<B>) -> $crate::NetResult<()> {
                match self {
                    $(
                        Self::$variant_name(v) => {
                            ($variant_ix as $ix_ty).encode_packet(pw)?;
                            v.encode_packet(pw)?;
                        }
                    ),*
                }

                Ok(())

            }
        }

        impl<'de> $crate::proto::DecodePacket<'de> for $name {
            fn decode_packet(pr: &mut $crate::MaplePacketReader<'de>) -> $crate::NetResult<Self> {
                let ix = <$ix_ty>::decode_packet(pr)?;
                Ok(match ix {
                    $(
                        $variant_ix => {
                            let v = <$variant_ty>::decode_packet(pr)?;
                            Self::$variant_name(v)
                        }
                    ),*
                    _ => return Err($crate::NetError::InvalidEnumDiscriminant(ix as usize))
                })
            }
        }

        impl $crate::proto::PacketLen for $name {
            const SIZE_HINT: Option<usize> = None;

            fn packet_len(&self) -> usize {
                match self {
                    $(
                        Self::$variant_name(v) => {
                            <$ix_ty>::SIZE_HINT.unwrap() + v.packet_len()
                        }
                    ),*
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::proto::{DecodePacket, EncodePacket};

    #[test]
    fn name() {
        maple_packet_enum!(
            TestChoice,
            u16,
            One(()) => 0,
            Two(u32) => 1,
        );

        let data = [TestChoice::One(()), TestChoice::Two(1337)];

        for d in data.iter() {
            let pkt = d.to_data().unwrap();
            let _dec = TestChoice::decode_from_data(&pkt).unwrap();

            //TODO find a way to compare: assert_eq!(dec.maple_into_inner(), d.maple_into_inner());
        }
    }

    #[test]
    fn code() {
        maple_enum_code!(
            Code,
            u8,
            A = 1,
            B = 2,
            C = 3
        );

        assert_eq!(Code::A, Code::A);
        let a: u8 = Code::A.into();
        assert_eq!(a, 1u8);
    }
}
