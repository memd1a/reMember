use std::{
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
    path::Path, rc::Rc,
};

use binrw::BinRead;

use memmap2::Mmap;

use image::{Rgba, RgbaImage};

use crate::{
    crypto::WzCrypto,
    l0::{WzDir, WzDirHeader, WzHeader, WzImgHeader},
    l1::{canvas::WzCanvas, obj::WzObject, prop::WzObj},
    version::{WzRegion, WzVersion},
};
pub trait WzIO: BufRead + Seek {}
impl<T> WzIO for T where T: BufRead + Seek {}

pub struct SubReader<'a, R> {
    inner: &'a mut R,
    offset: u64,
    size: u64,
}

fn bgra4_to_rgba8(v: u16) -> Rgba<u8> {
    let b = (v & 0x0F) as u8 * 16;
    let g = (v >> 4 & 0x0F) as u8 * 16;
    let r = (v >> 8 & 0x0F) as u8 * 16;
    let a = (v >> 12 & 0x0F) as u8 * 16;

    [r, g, b, a].into()
}

impl<'a, R> Read for SubReader<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<'a, R> BufRead for SubReader<'a, R>
where
    R: BufRead,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

// TODO this MUST be tested
impl<'a, R> Seek for SubReader<'a, R>
where
    R: Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let pos = match pos {
            SeekFrom::Current(p) => SeekFrom::Current(p),
            SeekFrom::End(p) => SeekFrom::End((self.offset + self.size) as i64 + p),
            SeekFrom::Start(p) => SeekFrom::Start(p + self.offset),
        };
        self.inner.seek(pos).map(|p| p - self.offset)
    }
}

impl<'a, R> SubReader<'a, R>
where
    R: Read + Seek,
{
    pub fn new(r: &'a mut R, offset: u64, size: u64) -> Self {
        Self {
            inner: r,
            offset,
            size,
        }
    }
}

pub struct WzImgReader<R> {
    r: R,
    crypto: Rc<WzCrypto>,
}

impl<R> WzImgReader<R>
where
    R: WzIO,
{
    pub fn read_root_obj(&mut self) -> anyhow::Result<WzObject> {
        self.r.rewind()?;
        Ok(WzObject::read_le_args(&mut self.r, &self.crypto)?)
    }

    pub fn read_obj(&mut self, obj: &WzObj) -> anyhow::Result<WzObject> {
        self.r.seek(SeekFrom::Start(obj.len.pos + 4))?;
        Ok(WzObject::read_le_args(&mut self.r, &self.crypto)?)
    }

    fn dechunk(&self, data: &mut [u8]) {
        //TODO use result here properly
        let mut i = 0;
        let mut j = 0;
        let len = data.len();

        while i < len {
            let chunk_size = u32::from_le_bytes(data[i..i + 4].try_into().unwrap()) as usize;
            if chunk_size >= 16_000 {
                dbg!(chunk_size);
                unimplemented!("Bad chunk size");
            }
            i += 4;

            data.copy_within(i..i + chunk_size, j);
            i += chunk_size;

            self.crypto
                .transform(data[j..j + chunk_size].as_mut().into());
            j += chunk_size;
        }
    }

    pub fn read_canvas(&mut self, canvas: &WzCanvas) -> anyhow::Result<image::RgbaImage> {
        let len = canvas.len.val as usize - 1;
        let off = canvas.len.pos + 4 + 1;
        self.r.seek(SeekFrom::Start(off))?;

        let buf = self.r.fill_buf()?;
        let hdr = u16::from_le_bytes(buf[..2].try_into().unwrap());

        let data = if hdr == 0x9C78 || hdr != 0xffff {
            let mut img_buf = Vec::with_capacity((canvas.width.0 * canvas.height.0 * 2) as usize);
            let mut sub = (&mut self.r).take(len as u64);
            let mut dec = flate2::bufread::ZlibDecoder::new(&mut sub);
            dec.read_to_end(&mut img_buf)?;
            img_buf
        } else {
            let mut img_buf = vec![0; len];
            self.r.read_exact(&mut img_buf)?;
            self.dechunk(&mut img_buf);
            img_buf
        };
        let w = canvas.width.0 as u32;
        let h = canvas.height.0 as u32;

        let data: &[u16] = bytemuck::cast_slice(&data);
        Ok(RgbaImage::from_fn(w, h, |x, y| {
            bgra4_to_rgba8(data[(x + y * w) as usize])
        }))
    }
}

#[derive(Debug)]
pub struct WzReader<R> {
    inner: R,
    crypto: Rc<WzCrypto>,
    data_offset: u64,
}

pub type WzReaderFile = WzReader<BufReader<File>>;

impl WzReaderFile {
    pub fn open_file(
        path: impl AsRef<Path>,
        region: WzRegion,
        version: WzVersion,
    ) -> anyhow::Result<Self> {
        Self::open(BufReader::new(File::open(path)?), region, version)
    }
}

pub type WzReaderMmap = WzReader<Cursor<Mmap>>;

impl WzReaderMmap {
    pub fn open_file_mmap(
        path: impl AsRef<Path>,
        region: WzRegion,
        version: WzVersion,
    ) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Self::open(Cursor::new(mmap), region, version)
    }
}

impl<R> WzReader<R>
where
    R: WzIO,
{
    pub fn open(mut rdr: R, region: WzRegion, ver: WzVersion) -> anyhow::Result<Self> {
        let hdr = WzHeader::read_le(&mut rdr)?;
        rdr.seek(SeekFrom::Start(hdr.data_offset as u64))?;

        let encrypted_version: u16 = u16::read_le(&mut rdr)?;
        if ver.encrypted_version() != encrypted_version {
            anyhow::bail!("Wrong version: {}", encrypted_version);
        }

        Ok(Self {
            inner: rdr,
            crypto: WzCrypto::from_region(region, ver, hdr.data_offset).into(),
            data_offset: hdr.data_offset as u64,
        })
    }

    pub fn read_root_dir(&mut self) -> anyhow::Result<WzDir> {
        // Skip encrypted version at the start
        self.read_dir(self.data_offset + 2)
    }

    pub fn read_dir_node(&mut self, hdr: &WzDirHeader) -> anyhow::Result<WzDir> {
        self.read_dir(hdr.offset.0 as u64)
    }

    fn read_dir(&mut self, offset: u64) -> anyhow::Result<WzDir> {
        self.inner.seek(SeekFrom::Start(offset))?;
        self.set_pos(offset)?;
        Ok(WzDir::read_le_args(&mut self.inner, &self.crypto)?)
    }

    pub fn img_reader<'a>(&'a mut self, hdr: &WzImgHeader) -> WzImgReader<SubReader<'a, R>> {
        let sub = SubReader::new(&mut self.inner, hdr.offset.0 as u64, hdr.blob_size.0 as u64);
        WzImgReader {
            r: sub,
            crypto: self.crypto.clone(),
        }
    }

    fn set_pos(&mut self, p: u64) -> io::Result<()> {
        self.inner.seek(SeekFrom::Start(p))?;
        Ok(())
    }
}
