use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::Path,
};

use binrw::BinRead;

use crate::{
    crypto::WzCrypto,
    l0::{WzDir, WzDirHeader, WzHeader, WzImgHeader},
    l1::obj::WzObject,
    version::{WzRegion, WzVersion},
};

struct SubReader<'a, R> {
    inner: &'a mut R,
    offset: u64,
    size: u64,
}

impl<'a, R> Read for SubReader<'a, R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
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

#[derive(Debug)]
pub struct WzReader<R> {
    inner: R,
    crypto: WzCrypto,
    data_offset: u64,
}

impl WzReader<File> {
    pub fn open_file(
        path: impl AsRef<Path>,
        region: WzRegion,
        version: WzVersion,
    ) -> anyhow::Result<Self> {
        Self::open(File::open(path)?, region, version)
    }
}

impl<R> WzReader<R>
where
    R: Read + Seek,
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
            crypto: WzCrypto::from_region(region, ver, hdr.data_offset),
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

    pub fn read_img(&mut self, hdr: &WzImgHeader) -> anyhow::Result<WzObject> {
        let mut sub = SubReader::new(&mut self.inner, hdr.offset.0 as u64, hdr.blob_size.0 as u64);
        sub.rewind()?;

        Ok(WzObject::read_le_args(&mut sub, &self.crypto)?)
    }

    fn set_pos(&mut self, p: u64) -> io::Result<()> {
        self.inner.seek(SeekFrom::Start(p))?;
        Ok(())
    }
}
