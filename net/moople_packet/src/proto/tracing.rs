use bytes::BufMut;

use crate::{MaplePacketWriter, MaplePacketReader, NetResult};

use super::{DecodePacket, EncodePacket};

pub struct TracingType {
    pub name: &'static str,
}

pub struct TracingField<'a, T> {
    pub field_name: &'static str,
    pub index: usize,
    pub ty: TracingType,
    pub value: &'a T,
}

impl<'a, T> EncodePacket for TracingField<'a, T>
where
    &'a T: EncodePacket,
{
    fn encode_packet<Buf: BufMut>(&self, pw: &mut MaplePacketWriter<Buf>) -> NetResult<()> {
        self.value.encode_packet(pw)
    }
}

pub struct TracingStruct {
    pub struct_name: &'static str,
}

pub enum TracingValue {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    Buf(Vec<u8>),
    Str(String),
    List(Option<usize>),
    Struct(TracingStruct),
}

macro_rules! impl_into_val {
    ($d:ident,$ty:ty) => {
        impl From<&$ty> for TracingValue {
            fn from(v: &$ty) -> TracingValue {
                TracingValue::$d(v.to_owned())
            }
        }
    };
}

impl_into_val!(Bool, bool);
impl_into_val!(I8, i8);
impl_into_val!(I16, i16);
impl_into_val!(I32, i32);
impl_into_val!(I64, i64);
impl_into_val!(I128, i128);

impl_into_val!(U8, u8);
impl_into_val!(U16, u16);
impl_into_val!(U32, u32);
impl_into_val!(U64, u64);
impl_into_val!(U128, u128);

impl_into_val!(F32, f32);
impl_into_val!(F64, f64);

impl HasTraceInformation for TracingValue {
    fn write_trace<TW: TracingWriter>(tw: &mut TW, v: Option<&Self>) {
        if let Some(v) = v {
            tw.write_value(v);
        }
    }
}

pub trait TracingWriter {
    fn write_value(&mut self, v: &TracingValue);

    fn start_struct(&mut self, strct: TracingStruct);
    fn end_struct(&mut self);

    fn start_list(&mut self, len: Option<usize>);
    fn end_list(&mut self);
}

pub trait HasTraceInformation {
    fn write_trace<TW: TracingWriter>(tw: &mut TW, v: Option<&Self>);
}

pub trait TracingDecodePacket<'de>: Sized {
    fn tracing_decode_packet<TW: TracingWriter>(
        pr: &mut MaplePacketReader<'de>,
        tw: &mut TW,
    ) -> NetResult<Self>;
}

pub trait TracingEncodePacket {
    fn tracing_encode_packet<Buf: BufMut, TW: TracingWriter>(
        &self,
        pw: &mut MaplePacketWriter<Buf>,
        tw: &mut TW,
    ) -> NetResult<()>;
}

impl<'de, T> TracingDecodePacket<'de> for T
where
    T: DecodePacket<'de> + HasTraceInformation,
{
    fn tracing_decode_packet<TW: TracingWriter>(
        pr: &mut MaplePacketReader<'de>,
        tw: &mut TW,
    ) -> NetResult<Self> {
        match T::decode_packet(pr) {
            Ok(val) => {
                T::write_trace(tw, Some(&val));
                NetResult::Ok(val)
            }
            Err(err) => {
                T::write_trace(tw, None);
                Err(err)
            }
        }
    }
}

impl<T> TracingEncodePacket for T
where
    T: EncodePacket + HasTraceInformation,
{
    fn tracing_encode_packet<Buf: BufMut, TW: TracingWriter>(
        &self,
        pw: &mut MaplePacketWriter<Buf>,
        tw: &mut TW,
    ) -> NetResult<()> {
        T::write_trace(tw, Some(self));
        self.encode_packet(pw)
    }
}

#[cfg(test)]
mod tests {

    use crate::{proto::DecodePacket, MaplePacketWriter, MaplePacketReader, NetResult};

    use super::{HasTraceInformation, TracingEncodePacket, TracingStruct};

    struct A {
        one: u8,
        two: u16,
        three: u32,
        four: u64,
        five: bool,
        six: String,
    }

    impl TracingEncodePacket for A {
        fn tracing_encode_packet<Buf: bytes::BufMut, TW: super::TracingWriter>(
            &self,
            _pw: &mut MaplePacketWriter<Buf>,
            _tw: &mut TW,
        ) -> NetResult<()> {
            Ok(())
        }
        /*fn traac<T: bytes::BufMut>(
            &self,
            pw: &mut crate::packet::MaplePacketWriter<T>,
        ) -> crate::NetResult<()> {
            (
                self.one,
                self.two,
                self.three,
                self.four,
                self.five,
                self.six.as_ref(),
            )
                .encode_packet(pw)
        }*/
    }

    impl<'de> DecodePacket<'de> for A {
        fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
            Ok(Self {
                one: <u8>::decode_packet(pr)?,
                two: <u16>::decode_packet(pr)?,
                three: <u32>::decode_packet(pr)?,
                four: <u64>::decode_packet(pr)?,
                five: <bool>::decode_packet(pr)?,
                six: <String>::decode_packet(pr)?,
            })
        }
    }

    impl HasTraceInformation for A {
        fn write_trace<TW: super::TracingWriter>(tw: &mut TW, _v: Option<&Self>) {
            tw.start_struct(TracingStruct { struct_name: "A" });

            tw.end_struct();
        }
    }

    #[test]
    fn trace_single() {}
}
