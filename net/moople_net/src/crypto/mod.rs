pub mod ig_cipher;
use cipher::{generic_array::GenericArray, typenum::U16};
use rand::{thread_rng, CryptoRng, Rng};

use crate::NetResult;

use self::{aes_cipher::MapleAESCipher, shanda_cipher::ShandaCipher};

pub mod aes_cipher;
pub mod header;
pub mod key;
pub mod shanda_cipher;

pub const ROUND_KEY_LEN: usize = 4;
pub const AES_KEY_LEN: usize = 32;
pub const AES_BLOCK_LEN: usize = 16;

pub const PACKET_HEADER_LEN: usize = 4;
pub type PacketHeader = [u8; PACKET_HEADER_LEN];

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
pub struct RoundKey(pub [u8; ROUND_KEY_LEN]);

impl Default for RoundKey {
    fn default() -> Self {
        key::INIT_ROUND_KEY
    }
}

impl From<[u8; ROUND_KEY_LEN]> for RoundKey {
    fn from(value: [u8; ROUND_KEY_LEN]) -> Self {
        Self(value)
    }
}

impl From<RoundKey> for u32 {
    fn from(value: RoundKey) -> Self {
        u32::from_le_bytes(value.0)
    }
}

impl From<u32> for RoundKey {
    fn from(value: u32) -> Self {
        Self(value.to_le_bytes())
    }
}

impl rand::Fill for RoundKey {
    fn try_fill<R: rand::Rng + ?Sized>(&mut self, rng: &mut R) -> Result<(), rand::Error> {
        let data: [u8; ROUND_KEY_LEN] = rng.gen();
        self.0 = data;
        Ok(())
    }
}

impl RoundKey {
    pub const fn zero() -> Self {
        RoundKey([0; ROUND_KEY_LEN])
    }

    pub fn get_random<R>(mut rng: R) -> Self
    where
        R: CryptoRng + Rng,
    {
        let mut zero = Self::zero();
        rng.fill(&mut zero);
        zero
    }

    pub fn update(&self) -> RoundKey {
        ig_cipher::inno_hash_n(&self.0, key::INIT_ROUND_KEY.into()).into()
    }

    pub fn expand(&self) -> GenericArray<u8, U16> {
        array_init::array_init(|i| self.0[i % ROUND_KEY_LEN]).into()
    }
}



pub struct MapleCrypto {
    maple_aes_cipher: MapleAESCipher,
    round_key: RoundKey,
}

impl Default for MapleCrypto {
    fn default() -> Self {
        Self::from_round_key(RoundKey::get_random(thread_rng()))
    }
}

impl MapleCrypto {
    pub fn new(key: [u8; AES_KEY_LEN], round_key: RoundKey) -> Self {
        Self {
            maple_aes_cipher: MapleAESCipher::new(&key).unwrap(),
            round_key,
        }
    }

    pub fn from_round_key(round_key: RoundKey) -> Self {
        Self::new(key::MAPLE_AES_KEY, round_key)
    }

    fn update_round_key(&mut self) {
        self.round_key = self.round_key.update();
    }

    pub fn encode_header(&self, length: u16, version: u16) -> PacketHeader {
        header::encode_header(self.round_key, length, version)
    }

    pub fn decode_header(&self, hdr: PacketHeader, version: u16) -> NetResult<u16> {
        header::decode_header(hdr, self.round_key, version)
    }

    /// Decrypt a chunk of data
    /// IMPORTANT: only call this with a full block of data, because the internal state updates
    pub fn encrypt(&mut self, data: &mut [u8]) {
        ShandaCipher::encrypt(data);
        self.maple_aes_cipher.crypt(self.round_key, data.into());
        self.update_round_key();
    }

    /// Encrypts a chunk of data
    /// IMPORTANT: only call this with a full block of data, because the internal state updates
    pub fn decrypt(&mut self, data: &mut [u8]) {
        self.maple_aes_cipher.crypt(self.round_key, data.into());
        self.update_round_key();
        ShandaCipher::decrypt(data);
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::{MapleCrypto, RoundKey};

    #[test]
    fn morph_sequence() {
        let key = RoundKey([1, 2, 3, 4]);
        assert_eq!(key.update().0, [123, 191, 164, 86]);
    }

    #[test]
    fn en_dec() {
        let key = RoundKey([1, 2, 3, 4]);

        let mut enc = MapleCrypto::from_round_key(key);
        let mut dec = MapleCrypto::from_round_key(key);
        let data = b"abcdef";

        let mut data_enc = *data;
        enc.encrypt(&mut data_enc);
        dec.decrypt(&mut data_enc);

        assert_eq!(*data, data_enc);
        assert_eq!(enc.round_key, dec.round_key);
    }

    #[test]
    fn en_dec_100() {
        let key = RoundKey([1, 2, 3, 4]);
        let mut enc = MapleCrypto::from_round_key(key);
        let mut dec = MapleCrypto::from_round_key(key);
        let data = b"abcdef".to_vec();

        for _ in 0..100 {
            let mut data_enc = data.clone();
            enc.encrypt(&mut data_enc);
            dec.decrypt(&mut data_enc);

            assert_eq!(*data, data_enc);
        }
    }
}
