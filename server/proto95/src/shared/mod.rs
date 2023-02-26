pub mod job;
pub mod char;
use std::net::Ipv4Addr;

use moople_derive::MooplePacket;
use moople_packet::{mark_maple_enum, proto::wrapped::MapleWrapped};
use num_enum::{TryFromPrimitive, IntoPrimitive};

pub type NameStr = arrayvec::ArrayString<13>;

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum Gender {
    #[default]
    Male = 0,
    Female = 1,
}
mark_maple_enum!(Gender);

#[derive(Debug, Eq, PartialEq, Copy, Clone, TryFromPrimitive, IntoPrimitive, Default)]
#[repr(u8)]
pub enum OptionGender {
    #[default]
    Male = 0,
    Female = 1,
    Unset = 10,
}

impl OptionGender {
    pub fn is_set(&self) -> bool {
        !self.is_unset()
    }

    pub fn is_unset(&self) -> bool {
        matches!(self, OptionGender::Unset)
    }
}

impl<T> From<Option<T>> for OptionGender
where
    T: Into<Gender>,
{
    fn from(value: Option<T>) -> Self {
        match value.map(Into::into) {
            None => OptionGender::Unset,
            Some(Gender::Female) => OptionGender::Female,
            Some(Gender::Male) => OptionGender::Male,
        }
    }
}

impl From<OptionGender> for Option<Gender> {
    fn from(val: OptionGender) -> Self {
        match val {
            OptionGender::Female => Some(Gender::Female),
            OptionGender::Male => Some(Gender::Male),
            OptionGender::Unset => None,
        }
    }
}
mark_maple_enum!(OptionGender);

#[derive(Debug, MooplePacket)]
pub struct Vec2 {
    x: i16,
    y: i16,
}

#[derive(Debug, MooplePacket)]
pub struct Rect {
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
}

#[derive(Debug, Clone)]
pub struct ServerAddr(pub Ipv4Addr);

impl MapleWrapped for ServerAddr {
    type Inner = [u8; 4];

    fn maple_into_inner(&self) -> Self::Inner {
        self.0.octets()
    }

    fn maple_from(v: Self::Inner) -> Self {
        Self(Ipv4Addr::from(v))
    }
}