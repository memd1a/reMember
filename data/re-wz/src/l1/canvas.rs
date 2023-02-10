use binrw::binrw;

use crate::{crypto::WzCrypto, ty::WzInt};

use super::prop::WzProperty;

#[binrw]
#[brw(little, import(crypto: &WzCrypto))]
#[derive(Debug)]
pub struct WzCanvas {
    unknown: u8,
    has_property: u8,
    #[brw(if(has_property.eq(&1)), args(crypto))]
    other_byte: Option<WzProperty>,
    width: WzInt,
    height: WzInt,
    depth: WzInt,
    scale: u8,
    unknown1: u32,
    len: u32,
    unknown2: u8,
}
