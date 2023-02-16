use std::io::Cursor;

use bytes::Buf;

use crate::{opcode::NetOpcode, NetResult, error::NetError};

#[derive(Debug)]
pub struct MaplePacketReader<'a> {
    inner: Cursor<&'a [u8]>,
}

impl<'a> MaplePacketReader<'a> {
    pub fn str_packet_len(s: &str) -> usize {
        // Len + data
        2 + s.len()
    }


    pub fn new(inner: &'a [u8]) -> Self {
        Self {
            inner: Cursor::new(inner),
        }
    }

    fn check_size_typed<T>(&self, n: usize) -> NetResult<()> {
        if self.inner.remaining() >= n {
            Ok(())
        } else {
            //TODO disable diagnostic for release builds
            Err(NetError::eof::<T>(self.inner.get_ref(), n))
        }
    }

    fn check_size(&self, n: usize) -> NetResult<()> {
        self.check_size_typed::<()>(n)
    }

    pub fn remaining_slice(&self) -> &'a [u8] {
        let p = self.inner.position() as usize;
        &self.inner.get_ref()[p..]
    }


    pub fn sub_reader(&self) -> Self {
        Self::new(self.remaining_slice())
    }

    pub fn commit_sub_reader(&mut self, sub_reader: Self) -> NetResult<()> {
        self.advance(sub_reader.inner.position() as usize)
    }

    pub fn advance(&mut self, n: usize) -> NetResult<()> {
        self.check_size(n)?;
        self.inner.advance(n);
        Ok(())
    }

    pub fn read_opcode<T: NetOpcode>(&mut self) -> NetResult<T> {
        let v = self.read_u16()?;
        T::get_opcode(v)
    }

    pub fn read_u8(&mut self) -> NetResult<u8> {
        self.check_size_typed::<u8>(1)?;
        Ok(self.inner.get_u8())
    }

    pub fn read_i8(&mut self) -> NetResult<i8> {
        self.check_size_typed::<i8>(1)?;
        Ok(self.inner.get_i8())
    }

    pub fn read_bool(&mut self) -> NetResult<bool> {
        self.check_size_typed::<bool>(1)?;
        Ok(self.read_u8()? != 0)
    }

    pub fn read_u16(&mut self) -> NetResult<u16> {
        self.check_size_typed::<u16>(2)?;
        Ok(self.inner.get_u16_le())
    }

    pub fn read_i16(&mut self) -> NetResult<i16> {
        self.check_size_typed::<i16>(2)?;
        Ok(self.inner.get_i16_le())
    }

    pub fn read_u32(&mut self) -> NetResult<u32> {
        self.check_size_typed::<u32>(4)?;
        Ok(self.inner.get_u32_le())
    }

    pub fn read_i32(&mut self) -> NetResult<i32> {
        self.check_size_typed::<i32>(4)?;
        Ok(self.inner.get_i32_le())
    }

    pub fn read_u64(&mut self) -> NetResult<u64> {
        self.check_size_typed::<u64>(8)?;
        Ok(self.inner.get_u64_le())
    }

    pub fn read_i64(&mut self) -> NetResult<i64> {
        self.check_size_typed::<i64>(8)?;
        Ok(self.inner.get_i64_le())
    }

    pub fn read_u128(&mut self) -> NetResult<u128> {
        self.check_size_typed::<u128>(16)?;
        Ok(self.inner.get_u128_le())
    }

    pub fn read_i128(&mut self) -> NetResult<i128> {
        self.check_size_typed::<i128>(16)?;
        Ok(self.inner.get_i128_le())
    }

    pub fn read_f32(&mut self) -> NetResult<f32> {
        self.check_size_typed::<f32>(4)?;
        Ok(self.inner.get_f32_le())
    }

    pub fn read_f64(&mut self) -> NetResult<f64> {
        self.check_size_typed::<f64>(8)?;
        Ok(self.inner.get_f64_le())
    }

    pub fn read_string(&mut self) -> NetResult<&'a str> {
        let n = self.read_u16()? as usize;
        let str_inner = self.read_bytes_inner::<&'a str>(n)?;
        Ok(std::str::from_utf8(str_inner)?)
    }

    pub fn read_string_limited(&mut self, limit: usize) -> NetResult<&'a str> {
        let n = self.read_u16()? as usize;
        if n >= limit {
            return Err(NetError::StringLimit(limit));
        }

        let str_inner = self.read_bytes_inner::<&'a str>(n)?;
        Ok(std::str::from_utf8(str_inner)?)
    }

    pub fn read_bytes(&mut self, n: usize) -> NetResult<&'a [u8]> {
        self.read_bytes_inner::<&'a [u8]>(n)
    }

    pub fn read_array<const N: usize>(&mut self) -> NetResult<[u8; N]> {
        let arr: [u8; N] = self.read_bytes_inner::<[u8; N]>(N)?.try_into().unwrap();

        Ok(arr)
    }

    #[inline]
    fn read_bytes_inner<T>(&mut self, n: usize) -> NetResult<&'a [u8]> {
        self.check_size_typed::<T>(n)?;
        let p = self.inner.position() as usize;
        // Size is already checked here
        let by = &self.inner.get_ref()[p..p+n];
        self.inner.advance(n);
        Ok(by)
    }
}
