use binrw::{binrw, NullString};

use crate::{
    crypto::WzCrypto,
    ty::{WzInt, WzOffset, WzStr, WzVec},
};

#[binrw]
#[brw(little)]
#[br(magic = b"PKG1")]
#[derive(Debug)]
pub struct WzHeader {
    pub file_size: u64,
    pub data_offset: u32,
    pub desc: NullString,
}

#[binrw]
#[brw(little, import_raw(crypto: &WzCrypto))]
#[derive(Debug)]
pub struct WzDir {
    #[brw(args(crypto))]
    pub entries: WzVec<WzDirNode>,
}

#[binrw]
#[brw(little, import(crypto: &WzCrypto))]
#[derive(Debug, Clone)]
pub struct WzImgHeader {
    #[brw(args(crypto))]
    pub name: WzStr,
    pub blob_size: WzInt,
    pub checksum: WzInt,
    #[brw(args(crypto))]
    pub offset: WzOffset,
}

#[binrw]
#[brw(little, import(crypto: &WzCrypto))]
#[derive(Debug, Clone)]
pub struct WzDirHeader {
    #[brw(args(crypto))]
    pub name: WzStr,
    pub blob_size: WzInt,
    pub checksum: WzInt,
    #[brw(args(crypto))]
    pub offset: WzOffset,
}

#[binrw]
#[brw(little, import(crypto: &WzCrypto))]
#[derive(Debug)]
pub enum WzDirNode {
    //01 XX 00 00 00 00 00 OFFSET (4 bytes)
    #[br(magic(1u8))]
    Nil([u8; 10]),
    // String at data_offset + Link.0
    #[br(magic(2u8))]
    Link(u32),
    #[br(magic(3u8))]
    Dir(#[brw(args(crypto))] WzDirHeader),
    #[brw(magic(4u8))]
    Img(#[brw(args(crypto))] WzImgHeader),
}
