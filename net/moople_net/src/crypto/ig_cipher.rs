use cipher::inout::InOutBuf;

use super::key;

fn shuffle(i: u8) -> u8 {
    key::ROUND_SHIFTING_KEY[i as usize]
}

fn update_key(key: u32, b: u8) -> u32 {
    let mut k = key.to_le_bytes();
    k[0] = k[0].wrapping_add(shuffle(k[1]).wrapping_sub(b));
    k[1] = k[1].wrapping_sub(k[2] ^ shuffle(b));
    k[2] ^= shuffle(k[3]).wrapping_add(b);
    k[3] = k[3].wrapping_sub(k[0].wrapping_sub(shuffle(b)));

    u32::from_le_bytes(k).rotate_left(3)
}

fn enc(data: u8, key: [u8; 4]) -> u8 {
    let v = data.rotate_right(4);
    // v(even bits) = (a << 1) & 0xAA(even bits)
    let even = (v & 0xAA) >> 1;
    // v(odd bits) = (a >> 1) & 0x55(odd bits) 
    let odd = (v & 0x55) << 1;

    let a = even | odd;
    a ^ shuffle(key[0])
}

fn dec(data: u8, key: [u8; 4]) -> u8 {
    let a = shuffle(key[0]) ^ data;
    let b = a << 1;

    let mut v = a;
    v >>= 1;
    v ^= b;
    v &= 0x55;
    v ^= b;
    v.rotate_left(4)
}

pub fn inno_hash_n<const N: usize>(data: &[u8; N], mut key: u32) -> u32 {
    for &b in data.iter() {
        key = update_key(key, b);
    }

    key
}

pub fn inno_hash(data: &[u8], mut key: u32) -> u32 {
    for &b in data.iter() {
        key = update_key(key, b);
    }

    key
}

pub fn inno_decrypt(buf: InOutBuf<u8>, key: &mut u32) {
    let buf = buf.into_out();
    for b in buf.iter_mut() {
        *b = dec(*b, key.to_le_bytes());
        *key = update_key(*key, *b);
    }
}

pub fn inno_encrypt(buf: InOutBuf<u8>, key: &mut u32) {
    let buf = buf.into_out();
    for b in buf.iter_mut() {
        *b = enc(*b, key.to_le_bytes());
        *key = update_key(*key, *b);
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::{ig_cipher::{dec, enc}, key};

    #[test]
    fn ig_dec_enc() {
        let key = key::INIT_ROUND_KEY.0;

        let v = dec(0x31, key);
        assert_eq!(v, 10);

        let d = enc(v, key);
        assert_eq!(d, 0x31);

        let v = dec(0xff, key);
        assert_eq!(enc(v, key), 0xff);
    }
}
