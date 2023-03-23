pub mod tree;
pub mod obj;
use binrw::{binread, binrw, BinWrite, FilePtr};

use crate::{crypto::WzCrypto, ty::WzStr};

pub mod canvas;
pub mod prop;

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct WzOffsetStr {
    pub offset: i32,
}

#[binread]
#[brw(little, import(crypto: &WzCrypto))]
#[derive(Debug)]
pub enum WzUOLStr {
    #[br(magic(0u8))]
    Str(#[brw(args(crypto))] WzStr),
    #[br(magic(0x73u8))]
    StrTypeName(#[brw(args(crypto))] WzStr),
    #[br(magic(1u8))]
    Offset(#[brw(args { inner: (crypto,) })] FilePtr<u32, WzStr>),
    #[br(magic(0x1bu8))]
    OffsetTypeName(#[brw(args { inner: (crypto,) })] FilePtr<u32, WzStr>),
}

impl WzUOLStr {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) | Self::StrTypeName(s) => s,
            Self::Offset(s) | Self::OffsetTypeName(s) => s.value.as_ref().unwrap(),
        }
        .as_str()
    }
}

impl BinWrite for WzUOLStr {
    type Args<'a> = (&'a WzCrypto,);

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        _writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        todo!()
    }
}
