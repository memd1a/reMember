use super::wrapped::PacketWrapped;
use bitflags::BitFlags;
use packed_struct::PackedStruct;

pub struct MapleBitFlags<T: BitFlags>(pub T);

impl<T: BitFlags> MapleBitFlags<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    pub fn cloned(inner: &T) -> Self {
        Self(T::from_bits(inner.bits()).unwrap())
    }
}

impl<T> PacketWrapped for MapleBitFlags<T>
where
    T: BitFlags,
{
    type Inner = T::Bits;

    fn packet_into_inner(&self) -> Self::Inner {
        self.0.bits()
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self(T::from_bits_truncate(v))
    }
}

#[macro_export]
macro_rules! mark_maple_bit_flags {
    ($ty:ty) => {
        impl $crate::proto::PacketWrapped for $ty {
            type Inner = $crate::proto::bits::MapleBitFlags<$ty>;

            fn packet_into_inner(&self) -> Self::Inner {
                Self::Inner::cloned(self)
            }

            fn packet_from(v: Self::Inner) -> Self {
                v.0
            }
        }
    };
}

pub struct MaplePacked<T: PackedStruct>(pub T);

impl<T> PacketWrapped for MaplePacked<T>
where
    T: PackedStruct + Clone,
{
    type Inner = T::ByteArray;

    fn packet_into_inner(&self) -> Self::Inner {
        self.0.pack().expect("pack")
    }

    fn packet_from(v: Self::Inner) -> Self {
        Self(T::unpack(&v).expect("unpack"))
    }
}

#[macro_export]
macro_rules! maple_mark_packed {
    ($ty:ty) => {
        impl $crate::proto::PacketWrapped for $ty {
            type Inner = $crate::proto::bits::MaplePacked<$ty>;

            fn packet_into_inner(&self) -> Self::Inner {
                //TODO find a more efficient way to do this, cloning the struct is not good
                // Maybe this should be a Transparent type instead of a wrapped
                // with different into and from type
                // in this case: into -> &T, from <- T
                $crate::proto::bits::MaplePacked(self.clone())
            }

            fn packet_from(v: Self::Inner) -> Self {
                v.0
            }
        }
    };
}
