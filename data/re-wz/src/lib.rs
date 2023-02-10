pub mod crypto;
pub mod file;
pub mod keys;
pub mod l0;
pub mod l1;
pub mod tree;
pub mod ty;
pub mod version;


#[cfg(test)]
mod tests {
    use crate::{file::WzReader, version::WzVersion};

    use super::*;

    #[test]
    fn load() -> anyhow::Result<()> {
        let mut item =  WzReader::open_file("../test_files/it.wz", version::WzRegion::GMS, WzVersion(95))?;
        dbg!(item.read_root_dir()?);
        Ok(())
    }
}
