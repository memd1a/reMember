use std::str::Utf8Error;

use arrayvec::{ArrayString, CapacityError};
use bytes::BufMut;

use crate::{DecodePacket, EncodePacket, MaplePacketReader, MaplePacketWriter, NetResult};

use super::PacketTryWrapped;

impl EncodePacket for String {
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        self.as_str().encode_packet(pw)
    }

    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        self.as_str().packet_len()
    }
}

impl<'de> DecodePacket<'de> for String {
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        Ok(<&'de str>::decode_packet(pr)?.to_string())
    }
}

impl<'de> DecodePacket<'de> for &'de str {
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        pr.read_string()
    }
}

impl<'a> EncodePacket for &'a str {
    fn encode_packet<B: BufMut>(&self, pw: &mut MaplePacketWriter<B>) -> NetResult<()> {
        pw.write_str(self);
        Ok(())
    }

    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        MaplePacketReader::str_packet_len(self)
    }
}

impl<const N: usize> EncodePacket for arrayvec::ArrayString<N> {
    fn encode_packet<T>(&self, pw: &mut MaplePacketWriter<T>) -> NetResult<()>
    where
        T: BufMut,
    {
        pw.write_str(self.as_str());
        Ok(())
    }

    const SIZE_HINT: Option<usize> = None;

    fn packet_len(&self) -> usize {
        self.len() + 2
    }
}

impl<'de, const N: usize> DecodePacket<'de> for arrayvec::ArrayString<N> {
    fn decode_packet(pr: &mut MaplePacketReader<'de>) -> NetResult<Self> {
        let s = pr.read_string_limited(N)?;
        Ok(arrayvec::ArrayString::from(s).unwrap())
    }
}

fn from_c_str<const N: usize>(b: &[u8; N]) -> Result<ArrayString<N>, Utf8Error> {
    let mut result = ArrayString::from_byte_string(b)?;
    if let Some(i) = &result.find('\0') {
        result.truncate(*i);
    }
    Ok(result)
}

/// A fixed string with the capacity of `N` bytes
/// If the len is less than `N` padding bytes 0 will be added
/// after the data
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub struct FixedPacketString<const N: usize>(pub arrayvec::ArrayString<N>);

impl<const N: usize> PacketTryWrapped for FixedPacketString<N> {
    type Inner = [u8; N];

    fn packet_into_inner(&self) -> Self::Inner {
        //TODO maybe this can be made more efficient
        let mut buf = [0; N];
        for (i, b) in self.0.as_bytes().iter().enumerate() {
            buf[i] = *b;
        }
        buf
    }

    fn packet_try_from(v: Self::Inner) -> NetResult<Self> {
        Ok(Self(from_c_str(&v)?))
    }
}

impl<'a, const N: usize> TryFrom<&'a str> for FixedPacketString<N> {
    type Error = CapacityError<&'a str>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        ArrayString::try_from(value).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use arrayvec::ArrayString;

    use crate::proto::tests::enc_dec_test_all;

    use super::FixedPacketString;

    #[test]
    fn string() {
        // String / str
        // String uses &str so no need to test that
        enc_dec_test_all(["".to_string(), "AAAAAAAAAAA".to_string(), "\0".to_string()]);
    }

    #[test]
    fn array_string() {
        enc_dec_test_all::<ArrayString<11>>([
            "".try_into().unwrap(),
            "AAAAAAAAAAA".try_into().unwrap(),
            "\0".try_into().unwrap(),
        ]);
    }

    #[test]
    fn fixed_string() {
        enc_dec_test_all::<FixedPacketString<11>>([
            "".try_into().unwrap(),
            "AAAAAAAAAAA".try_into().unwrap(),
            "a".try_into().unwrap(),
        ]);
    }
}
