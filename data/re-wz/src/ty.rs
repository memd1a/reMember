use std::{
    io::{Read, Seek},
    ops::Neg, num::Wrapping,
};

use binrw::{binrw, BinRead, BinWrite, VecArgs};

use crate::crypto::WzCrypto;

pub type RefWzCrypto<'a> = (&'a WzCrypto,);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WzInt(pub i32);

impl BinRead for WzInt {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        Ok(Self(match i8::read_options(reader, endian, args)? {
            -128 => i32::read_options(reader, endian, args)?,
            flag => flag as i32,
        }))
    }
}

impl BinWrite for WzInt {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        match i8::try_from(self.0) {
            Ok(v) if v != -128 => v.write_options(writer, endian, args),
            _ => {
                (-128i8).write_options(writer, endian, args)?;
                (self.0).write_options(writer, endian, args)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WzLong(pub i64);

impl BinRead for WzLong {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        Ok(Self(match i8::read_options(reader, endian, args)? {
            -128 => i64::read_options(reader, endian, args)?,
            flag => flag as i64,
        }))
    }
}

impl BinWrite for WzLong {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        match i8::try_from(self.0) {
            Ok(v) if v != -128 => v.write_options(writer, endian, args),
            _ => {
                (-128i8).write_options(writer, endian, args)?;
                (self.0).write_options(writer, endian, args)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WzF32(pub f32);

impl BinRead for WzF32 {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        Ok(Self(f32::from_bits(
            WzInt::read_options(reader, endian, args)?.0 as u32,
        )))
    }
}

impl BinWrite for WzF32 {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        WzInt(self.0.to_bits() as i32).write_options(writer, endian, args)
    }
}

fn xor_mask_ascii(data: &mut [u8]) {
    let mut mask = Wrapping(0xAAu8);
    for b in data.iter_mut() {
        *b ^= mask.0;
        mask += 1;
    }
}

fn xor_mask_unicode(data: &mut [u16]) {
    let mut mask = Wrapping(0xAAAA);
    for b in data.iter_mut() {
        *b ^= mask.0;
        mask += 1;
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct WzOffsetStr {
    pub offset: i32,
}

#[derive(Debug, Clone)]
pub enum WzStr {
    ASCII(String),
    Unicode(Vec<u16>),
}

impl WzStr {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::ASCII(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

impl BinRead for WzStr {
    type Args<'a> = RefWzCrypto<'a>;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let (is_ascii, len) = {
            let flag = i8::read_options(reader, endian, ())?;
            match flag {
                // Negative is ASCII
                -128 => (true, i32::read_options(reader, endian, ())? as usize),
                ln if ln <= 0 => (true, -ln as usize),
                //Positive is unicode
                127 => (false, i32::read_options(reader, endian, ())? as usize),
                ln => (false, ln as usize),
            }
        };

        match is_ascii {
            true => {
                let mut data = vec![0; len];
                reader.read_exact(&mut data)?;
                xor_mask_ascii(&mut data);
                args.0.transform(data.as_mut_slice().into());
                let data = String::from_utf8(data).unwrap();
                Ok(WzStr::ASCII(data))
            }
            false => {
                let mut data = vec![0u16; len];
                reader.read_exact(bytemuck::cast_slice_mut(data.as_mut_slice()))?;
                xor_mask_unicode(&mut data);
                args.0
                    .transform(bytemuck::cast_slice_mut(data.as_mut_slice()).into());

                Ok(WzStr::Unicode(data))
            }
        }
    }
}

impl BinWrite for WzStr {
    type Args<'a> = RefWzCrypto<'a>;

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        match self {
            Self::ASCII(ref data) => {
                let n = data.len();
                if n >= 128 {
                    i8::MIN.write_options(writer, endian, ())?;
                    (n as i32).neg().write_options(writer, endian, ())?;
                } else {
                    (n as i8).neg().write_options(writer, endian, ())?;
                }

                //TODO: xor + encrypt buffer

                data.as_bytes().write_options(writer, endian, ())?;
            }
            Self::Unicode(ref data) => {
                let n = data.len();
                if n >= 127 {
                    i8::MAX.write_options(writer, endian, ())?;
                    (n as i32).write_options(writer, endian, ())?;
                } else {
                    (n as i8).write_options(writer, endian, ())?;
                }

                data.write_options(writer, endian, ())?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WzVec<B>(pub Vec<B>);

impl<B> BinRead for WzVec<B>
where
    B: BinRead + 'static,
    for<'a> B::Args<'a>: Clone,
{
    type Args<'a> = B::Args<'a>;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let n = WzInt::read_options(reader, endian, ())?;
        Ok(Self(Vec::read_options(
            reader,
            endian,
            VecArgs {
                count: n.0 as usize,
                inner: args,
            },
        )?))
    }
}

impl<B> BinWrite for WzVec<B>
where
    B: BinWrite + 'static,
    for<'a> B::Args<'a>: Clone,
{
    type Args<'a> = B::Args<'a>;

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        WzInt(self.0.len() as i32).write_options(writer, endian, ())?;
        self.0.write_options(writer, endian, args)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WzOffset(pub u32);

impl BinRead for WzOffset {
    type Args<'a> = RefWzCrypto<'a>;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let pos = reader.stream_position()? as u32;
        let v = u32::read_options(reader, endian, ())?;
        let offset = args.0.decrypt_offset(v, pos);
        Ok(Self(offset))
    }
}

impl BinWrite for WzOffset {
    type Args<'a> = RefWzCrypto<'a>;

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let pos = writer.stream_position()? as u32;
        let enc_off = args.0.encrypt_offset(self.0, pos);
        enc_off.write_options(writer, endian, ())
    }
}
