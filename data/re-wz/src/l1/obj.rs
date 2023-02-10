use binrw::{BinRead, BinWrite};

use crate::crypto::WzCrypto;

use super::{canvas::WzCanvas, prop::WzProperty, WzUOLStr};

#[derive(Debug)]
pub enum WzObject {
    Property(WzProperty),
    Canvas(WzCanvas),
    /*
        TODO:

    Convex2D(Convex2D),
    Vector2D(Vec2D),
    UOL(WzUOL),
    SoundDX8(SoundDX8),*/
}

impl BinRead for WzObject {
    type Args<'a> = &'a WzCrypto;

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let ty_name = WzUOLStr::read_options(reader, endian, (args,))?;
        let ty_name: &str = ty_name.as_str().unwrap();
        Ok(match ty_name {
            "Property" => Self::Property(WzProperty::read_options(reader, endian, (args,))?),
            "Canvas" => Self::Canvas(WzCanvas::read_options(reader, endian, (args,))?),
            //"Shape2D#Vector2D" => Ok(WzObject::Vector2D(f.bin_read()?)),
            // TODO: make this a real error
            _ => {
                panic!("Invalid obj: {ty_name}")
            }
        })
    }
}

impl BinWrite for WzObject {
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
