use binrw::{binread, PosValue};

use crate::{crypto::WzCrypto, ty::WzInt};

use super::prop::WzProperty;

#[binread]
#[br(little, import(crypto: &WzCrypto))]
#[derive(Debug)]
pub struct WzCanvas {
    pub unknown: u8,
    pub has_property: u8,
    #[br(if(has_property.eq(&1)), args(crypto))]
    pub other_byte: Option<WzProperty>,
    pub width: WzInt,
    pub height: WzInt,
    pub depth: WzInt,
    pub scale: u8,
    pub unknown1: u32,
    pub len: PosValue<u32>,
}
