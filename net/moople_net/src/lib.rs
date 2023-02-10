use moople_packet::{NetError, NetResult};
pub use session::{MapleSession, SessionTransport};

pub mod codec;
pub mod crypto;
pub mod session;

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
