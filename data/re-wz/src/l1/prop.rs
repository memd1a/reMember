use std::io::{Read, Seek};

use binrw::{binread, binrw, BinRead, BinWrite, PosValue};

use crate::{
    crypto::WzCrypto,
    ty::{WzF32, WzInt, WzLong, WzVec},
};

use super::WzUOLStr;

#[binread]
#[br(little)]
#[derive(Debug)]
pub struct WzObjRef {
    pub len: PosValue<u32>,
}

impl BinWrite for WzObjRef {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        _writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct WzObj {
    pub len: PosValue<u32>,
}

impl BinRead for WzObj {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let len = PosValue::<u32>::read_options(reader, endian, args)?;
        reader.seek(std::io::SeekFrom::Current(len.val as i64))?;

        Ok(Self { len })
    }
}

impl BinWrite for WzObj {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        self.len.val.write_options(writer, endian, args)?;
        //TODO add data for object here
        Ok(())
    }
}

#[binrw]
#[brw(little, import(crypto: &WzCrypto))]
#[derive(Debug)]
pub enum WzValue {
    #[br(magic(0u8))]
    Null,

    // Short
    #[br(magic(2u8))]
    Short1(i16),
    #[br(magic(11u8))]
    Short2(i16),

    // Int
    #[br(magic(3u8))]
    Int1(WzInt),
    #[br(magic(19u8))]
    Int2(WzInt),

    // Long
    #[br(magic(20u8))]
    Long(WzLong),

    // Floats
    #[br(magic(4u8))]
    F32(WzF32),
    #[br(magic(5u8))]
    F64(f64),

    #[br(magic(8u8))]
    Str(#[brw(args(crypto))] WzUOLStr),

    #[br(magic(9u8))]
    Obj(WzObj),
}

#[binrw]
#[brw(little, import(crypto: &WzCrypto))]
#[derive(Debug)]
pub struct WzPropertyEntry {
    #[brw(args(crypto))]
    pub name: WzUOLStr,
    #[brw(args(crypto))]
    pub val: WzValue,
}

#[binrw]
#[brw(little, import(crypto: &WzCrypto))]
#[derive(Debug)]
pub struct WzProperty {
    pub unknown: u16,
    #[brw(args(crypto))]
    pub entries: WzVec<WzPropertyEntry>,
}


#[binrw]
#[brw(little, import(crypto: &WzCrypto))]
#[derive(Debug)]
pub struct WzUOL {
    pub unknown: u8,
    #[brw(args(crypto))]
    pub entries: WzUOLStr,
}


#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct WzVector2D {
    pub x: WzInt,
    pub y: WzInt
}