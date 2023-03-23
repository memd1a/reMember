use crate::{error::NetError, NetResult, proto::MapleWrapped};

pub trait NetOpcode: TryFrom<u16> + Into<u16> + Copy + Clone {
    fn get_opcode(v: u16) -> NetResult<Self> {
        Self::try_from(v).map_err(|_| NetError::InvalidOpcode(v))
    }
}

impl NetOpcode for u16 {}

pub trait HasOpcode {
    type OP: NetOpcode;

    const OPCODE: Self::OP;
}

#[derive(Debug, Default)]
pub struct WithOpcode<const OP: u16, T>(pub T);
impl<const OP: u16, T> HasOpcode for WithOpcode<OP, T> {
    type OP = u16;

    const OPCODE: Self::OP = OP;
}

impl<const OP: u16, T: Clone> MapleWrapped for WithOpcode<OP, T> {
    type Inner = T;

    fn maple_into_inner(&self) -> Self::Inner {
        //TODO clone is not right
        self.0.clone()
    }

    fn maple_from(v: Self::Inner) -> Self {
        Self(v)
    }
}


#[macro_export]
macro_rules! packet_opcode {
    ($packet_ty:ty, $op:path, $ty:ty) => {
        impl $crate::HasOpcode for $packet_ty {
            type OP = $ty;


            const OPCODE: Self::OP = $op;
        }
    };
    ($packet_ty:ty, $ty:ident::$op:ident) => {
        impl $crate::HasOpcode for $packet_ty {
            type OP = $ty;


            const OPCODE: Self::OP = $ty::$op;
        }
    }
}
