use moople_packet::NetError;

use crate::NetResult;

use super::{PacketHeader, RoundKey, PACKET_HEADER_LEN};


pub const fn invert_version(ver: u16) -> u16 {
    (-((ver + 1) as i16)) as u16
}


/// Small helper to work with high low words in a 32 bit integer
struct HiLo32 {
    high: u16,
    low: u16,
}

impl HiLo32 {
    fn from_low_high(high: u16, low: u16) -> Self {
        Self { high, low }
    }

    fn from_le_bytes(b: [u8; 4]) -> Self {
        let low = u16::from_le_bytes([b[0], b[1]]);
        let high = u16::from_le_bytes([b[2], b[3]]);
        Self { high, low }
    }

    fn to_le_bytes(&self) -> [u8; 4] {
        let mut result = [0; PACKET_HEADER_LEN];
        result[0..2].copy_from_slice(&self.low.to_le_bytes());
        result[2..4].copy_from_slice(&self.high.to_le_bytes());
        result
    }
}

pub fn decode_header(
    hdr: PacketHeader,
    key: RoundKey,
    version: u16,
) -> NetResult<u16> {
    let key = key.0;
    let v = HiLo32::from_le_bytes(hdr);
    let key_high = u16::from_le_bytes([key[2], key[3]]);
    let len = v.low ^ v.high;
    let hdr_key = v.low ^ version;

    if hdr_key != key_high {
        return Err(NetError::InvalidHeader{
            len,
            key: hdr_key,
            expected_key: key_high,
        });
    }

    Ok(len)
}

pub fn encode_header(key: RoundKey, length: u16, version: u16) -> PacketHeader {
    let key = key.0;
    let key_high = u16::from_le_bytes([key[2], key[3]]);
    let low = key_high ^ version;
    let hilo = HiLo32::from_low_high(low ^ length, low);
    hilo.to_le_bytes()
}

#[cfg(test)]
mod tests {
    use crate::crypto::{
        header::{decode_header, encode_header, invert_version},
        PacketHeader, RoundKey,
    };

    const V65: u16 = invert_version(65);
    const V83: u16 = invert_version(83);

    fn enc(key: RoundKey, len: u16, v: u16) -> PacketHeader {
        encode_header(key, len, v)
    }

    fn dec(key: RoundKey, hdr: PacketHeader, v: u16) -> u16 {
        decode_header(hdr, key, v).unwrap()
    }

    const KEY: RoundKey = RoundKey([82, 48, 120, 232]);
    const KEY2: RoundKey = RoundKey([82, 48, 120, 89]);

    #[test]
    fn header_enc_dec() {
        let tests = [
            (44, [198, 23, 234, 23], KEY, V65),
            (24, [212, 166, 204, 166], KEY2, V83),
            (
                627,
                [200, 140, 187, 142],
                KEY2.update(),
                V83,
            ),
        ];

        for (ln, ex, key, v) in tests {
            assert_eq!(enc(key, ln, v), ex);
            assert_eq!(dec(key, ex, v), ln)
        }
    }

    #[test]
    fn client_hdr_test() {
        let ex_hdr = [0x29, 0xd2, 0x2b, 0xd2];
        let iv_enc = RoundKey([70, 114, 122, 210]);
        let ln = decode_header(ex_hdr, iv_enc, 83).unwrap();
        assert_eq!(ln, 2);
    }
}
