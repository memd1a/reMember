use array_init::try_array_init;
use bytes::BufMut;
use either::Either;

use crate::{MaplePacketReader, MaplePacketWriter, NetResult};

use super::{DecodePacket, EncodePacket, PacketLen};

impl<'de> DecodePacket<'de> for () {
    fn decode_packet(_pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        Ok(())
    }
}

impl EncodePacket for () {
    fn encode_packet<B: BufMut>(&self, _pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        Ok(())
    }
}

impl PacketLen for () {
    const SIZE_HINT: Option<usize> = Some(0);

    fn packet_len(&self) -> usize {
        0
    }
}

impl<A, B> EncodePacket for Either<A, B>
where
    A: EncodePacket,
    B: EncodePacket,
{
    fn encode_packet<T: BufMut>(&self, pw: &mut MaplePacketWriter<T>) -> NetResult<()> {
        match self {
            Either::Left(a) => a.encode_packet(pw),
            Either::Right(b) => b.encode_packet(pw),
        }
    }
}

impl<A, B> PacketLen for Either<A, B>
where
    A: PacketLen,
    B: PacketLen,
{
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        match self {
            Either::Left(l) => l.packet_len(),
            Either::Right(r) => r.packet_len(),
        }
    }
}

pub struct OptionTail<T>(Option<T>);

impl<T> EncodePacket for OptionTail<T>
where
    T: EncodePacket,
{
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        if let Some(val) = self.0.as_ref() {
            val.encode_packet(pw)?;
        }
        Ok(())
    }
}

impl<'de, T> DecodePacket<'de> for OptionTail<T>
where
    T: DecodePacket<'de>,
{
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let mut sub_reader = pr.sub_reader();
        Ok(Self(T::decode_packet(&mut sub_reader).ok()))
    }
}
impl EncodePacket for String {
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        pw.write_str(self);
        Ok(())
    }
}

impl<'de> DecodePacket<'de> for String {
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        Ok(pr.read_string()?.to_string())
    }
}

impl PacketLen for String {
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        self.as_str().packet_len()
    }
}

macro_rules! impl_dec {
    ($ty:ty, $dec:path) => {
        impl<'de> DecodePacket<'de> for $ty {
            fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
                $dec(pr)
            }
        }
    };
}

macro_rules! impl_enc {
    ($ty:ty, $enc:path) => {
        impl EncodePacket for $ty {
            fn encode_packet<B: bytes::BufMut>(
                &self,
                pw: &mut MaplePacketWriter<B>,
            ) -> NetResult<()> {
                $enc(pw, *self);
                Ok(())
            }
        }
    };
}

macro_rules! impl_len {
    ($ty:ty) => {
        impl PacketLen for $ty {
            const SIZE_HINT: Option<usize> = Some(std::mem::size_of::<$ty>());

            fn packet_len(&self) -> usize {
                std::mem::size_of::<$ty>()
            }
        }
    };
}

macro_rules! impl_tracing {
    ($ty:ty) => {
        impl crate::proto::tracing::HasTraceInformation for $ty {
            fn write_trace<TW: crate::proto::tracing::TracingWriter>(
                tw: &mut TW,
                v: Option<&Self>,
            ) {
                match v {
                    Some(v) => {
                        let tracing_val: crate::proto::tracing::TracingValue = v.into();
                        crate::proto::tracing::TracingValue::write_trace(tw, Some(&tracing_val));
                    }
                    _ => {}
                }
            }
        }
    };
}

macro_rules! impl_dec_enc {
    ($ty:ty, $dec:path, $enc:path) => {
        impl_dec!($ty, $dec);
        impl_enc!($ty, $enc);
        impl_len!($ty);
        impl_tracing!($ty);
    };
}

impl_dec_enc!(
    bool,
    MaplePacketReader::read_bool,
    MaplePacketWriter::write_bool
);
impl_dec_enc!(u8, MaplePacketReader::read_u8, MaplePacketWriter::write_u8);
impl_dec_enc!(i8, MaplePacketReader::read_i8, MaplePacketWriter::write_i8);
impl_dec_enc!(
    u16,
    MaplePacketReader::read_u16,
    MaplePacketWriter::write_u16
);
impl_dec_enc!(
    u32,
    MaplePacketReader::read_u32,
    MaplePacketWriter::write_u32
);
impl_dec_enc!(
    u64,
    MaplePacketReader::read_u64,
    MaplePacketWriter::write_u64
);
impl_dec_enc!(
    u128,
    MaplePacketReader::read_u128,
    MaplePacketWriter::write_u128
);
impl_dec_enc!(
    i16,
    MaplePacketReader::read_i16,
    MaplePacketWriter::write_i16
);
impl_dec_enc!(
    i32,
    MaplePacketReader::read_i32,
    MaplePacketWriter::write_i32
);
impl_dec_enc!(
    i64,
    MaplePacketReader::read_i64,
    MaplePacketWriter::write_i64
);
impl_dec_enc!(
    i128,
    MaplePacketReader::read_i128,
    MaplePacketWriter::write_i128
);
impl_dec_enc!(
    f32,
    MaplePacketReader::read_f32,
    MaplePacketWriter::write_f32
);
impl_dec_enc!(
    f64,
    MaplePacketReader::read_f64,
    MaplePacketWriter::write_f64
);



impl<'de, const N: usize, T: DecodePacket<'de>> DecodePacket<'de> for [T; N] {
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        try_array_init(|_| T::decode_packet(pr))
    }
}

impl<const N: usize, T: EncodePacket> EncodePacket for [T; N] {
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        for v in self.iter() {
            v.encode_packet(pw)?;
        }
        Ok(())
    }
}

const fn mul(sz: Option<usize>, n: usize) -> Option<usize> {
    match sz {
        Some(sz) => Some(sz * n),
        _ => None,
    }
}

impl<const N: usize, T: PacketLen> PacketLen for [T; N] {
    const SIZE_HINT: Option<usize> = mul(T::SIZE_HINT, N);

    fn packet_len(&self) -> usize {
        self.iter().map(|v| v.packet_len()).sum()
    }
}

impl<D: EncodePacket> EncodePacket for Vec<D> {
    fn encode_packet<T: BufMut>(&self, pw: &mut MaplePacketWriter<T>) -> NetResult<()> {
        for v in self.iter() {
            v.encode_packet(pw)?;
        }

        Ok(())
    }
}

impl<D: PacketLen> PacketLen for Vec<D> {
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        self.iter().map(|v| v.packet_len()).sum()
    }
}

impl<D: EncodePacket> EncodePacket for Option<D> {
    fn encode_packet<T: BufMut>(&self, pw: &mut MaplePacketWriter<T>) -> NetResult<()> {
        if let Some(ref v) = self {
            v.encode_packet(pw)?;
        }

        Ok(())
    }
}

impl<D: PacketLen> PacketLen for Option<D> {
    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        self.as_ref().map(|v| v.packet_len()).unwrap_or(0)
    }
}