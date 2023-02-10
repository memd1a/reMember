use bytes::BytesMut;
use either::Either;
use moople_derive::MaplePacket;

use moople_packet::{proto::conditional::{CondOption, CondEither}, DecodePacket, EncodePacket, MaplePacketWriter, PacketLen};

#[derive(MaplePacket)]
pub struct Packet {
    name: u8,
    bitmask: u16,
}

#[derive(MaplePacket)]
pub struct Packet2(u8, u16);

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum TestOpcode {
    Action1 = 1,
}

impl From<TestOpcode> for u16 {
    fn from(val: TestOpcode) -> Self {
        val as u16
    }
}

impl TryFrom<u16> for TestOpcode {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TestOpcode::Action1),
            _ => Err(format!("Invalid test opcode: {value}")),
        }
    }
}

//impl NetOpcode for TestOpcode {}

#[derive(MaplePacket, Debug, PartialEq, Eq)]
pub struct Packet3<'a> {
    name: &'a str,
    bitmask: u16,
}

fn check_name_even(name: &str) -> bool {
    name.len() % 2 == 0
}

#[derive(MaplePacket, Debug, PartialEq, Eq)]
pub struct Packet4<'a, T> {
    name: &'a str,
    #[maple_packet(skip_if(field = "name", cond = "check_name_even"))]
    bitmask: CondOption<u16>,
    val: T,
}

fn check_n_even(n: &u32) -> bool {
    n%2 == 0
}

#[derive(MaplePacket, Debug, PartialEq, Eq)]
pub struct Packet5 {
    n: u32,
    #[maple_packet(either(field = "n", cond = "check_n_even"))]
    either: CondEither<String, bool>,
}

fn test_encode_decode<'de, T>(data: T, buf: &'de mut BytesMut)
where
    T: EncodePacket + DecodePacket<'de> + PartialEq + std::fmt::Debug,
{
    let mut pw = MaplePacketWriter::new(buf);
    data.encode_packet(&mut pw).expect("must encode");


    let inner = pw.into_inner();
    let cmp = T::decode_from_data(inner).expect("must decode");
    assert_eq!(data, cmp);
}

macro_rules! test_encode_decode {
    ($d:expr) => {
        let mut data = BytesMut::new();
        $crate::test_encode_decode($d, &mut data);
    };
}

fn main() {
    assert_eq!(Packet::SIZE_HINT, Some(3));
    assert_eq!(Packet3::SIZE_HINT, None);


    test_encode_decode!(Packet3 {
        name: "aaa",
        bitmask: 1337,
    });

    test_encode_decode!(Packet4 {
        name: "aaa",
        bitmask: CondOption(None),
        val: 1337u16,
    });
    test_encode_decode!(Packet4 {
        name: "aaaa",
        bitmask: CondOption(Some(1337)),
        val: 1337u16,
    });


    test_encode_decode!(Packet5 {
        n: 2,
        either: CondEither(Either::Left("ABC".to_string()))
    });

    test_encode_decode!(Packet5 {
        n: 1,
        either: CondEither(Either::Right(false))
    });
}
