use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf}
};

use game_data::map;
use proto95::id::MapId;

#[derive(Debug)]
pub struct MetaData {
    pub maps0: BTreeMap<i64, map::Map>,
}

pub type FieldMeta = &'static map::Map;

impl MetaData {
    fn load_from_file<T: serde::de::DeserializeOwned>(file: impl AsRef<Path>) -> anyhow::Result<T> {
        dbg!(file.as_ref().to_str());
        let file = File::open(file)?;
        Ok(bincode::deserialize_from(file)?)
    }

    pub fn load_from_dir(dir: PathBuf) -> anyhow::Result<Self> {
        Ok(Self {
            maps0: Self::load_from_file(dir.join("maps0.rbin"))?,
        })
    }
}

#[derive(Debug)]
pub struct MetaService {
    meta_data: MetaData,
}

impl MetaService {
    pub fn new(meta_data: MetaData) -> Self {
        Self { meta_data }
    }

    pub fn load_from_dir(dir: PathBuf) -> anyhow::Result<Self> {
        Ok(Self::new(MetaData::load_from_dir(dir)?))
    }

    pub fn get_field_data(&'static self, field_id: MapId) -> Option<FieldMeta> {
        self.meta_data.maps0.get(&(field_id.0 as i64))
    }
}
