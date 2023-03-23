use bitflags::BitFlags;
use packed_struct::PackedStruct;
use super::wrapped::MapleWrapped;

pub struct MapleBitFlags<T: BitFlags>(pub T);

impl<T: BitFlags> MapleBitFlags<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
    
    pub fn cloned(inner: &T) -> Self {
        Self(T::from_bits(inner.bits()).unwrap())
    }
}

impl<T> MapleWrapped for MapleBitFlags<T>  where T: BitFlags {
    type Inner = T::Bits;

    fn maple_into_inner(&self) -> Self::Inner {
        self.0.bits()
    }

    fn maple_from(v: Self::Inner) -> Self {
        Self(T::from_bits_truncate(v))
    }
}

#[macro_export]
macro_rules! mark_maple_bit_flags {
    ($ty:ty) => {
        impl $crate::proto::MapleWrapped for $ty {
            type Inner = $crate::proto::bits::MapleBitFlags<$ty>;

            fn maple_into_inner(&self) -> Self::Inner {
                Self::Inner::cloned(self)
            }
    
            fn maple_from(v: Self::Inner) -> Self {
                v.0
            }
        }
    }
}


pub struct MaplePacked<T: PackedStruct>(pub T);

impl<T> MapleWrapped for MaplePacked<T>  where T: PackedStruct + Clone {
    type Inner = T::ByteArray;

    fn maple_into_inner(&self) -> Self::Inner {
        self.0.pack().unwrap()
    }

    fn maple_from(v: Self::Inner) -> Self {
        Self(T::unpack(&v).unwrap())
    }
}

#[macro_export]
macro_rules! maple_mark_packet {
    ($ty:ty) => {
        impl $crate::proto::MapleWrapped for $ty {
            type Inner = $crate::proto::bits::MaplePacked<$ty>;

            fn maple_into_inner(&self) -> Self::Inner {
                //TODO find a more efficient way to do this, cloning the struct is not good
                // Maybe this should be a Transparent type instead of a wrapped
                // with different into and from type
                // in this case: into -> &T, from <- T
                $crate::proto::bits::MaplePacked(
                    self.clone()
                )
            }
    
            fn maple_from(v: Self::Inner) -> Self {
                v.0
            }
        }
    }
}