use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

fn get_file_hash(f: &mut File) -> anyhow::Result<String> {
    let mut hasher = Sha256::new();
    std::io::copy(f, &mut hasher)?;

    let result = hasher.finalize();
    Ok(hex::encode(&result))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileMeta {
    pub name: String,
    pub version: String,
    pub hash: String,
}

impl FileMeta {
    pub fn from_path(p: impl AsRef<Path>) -> anyhow::Result<Self> {
        let p = p.as_ref();
        let name = p
            .file_name()
            .ok_or_else(|| anyhow!("Must have file name"))?
            .to_string_lossy()
            .to_string();

        let mut file = File::open(p)?;
        let hash = get_file_hash(&mut file)?;

        Ok(Self {
            name,
            version: "1.0".to_string(),
            hash,
        })
    }

    pub fn should_update(&self, remote_meta: &FileMeta) -> bool {
        self.hash != remote_meta.hash
    }
}

pub struct FileEntry {
    pub path: PathBuf,
    pub meta: FileMeta,
}

impl FileEntry {
    pub fn from_path(p: impl AsRef<Path>) -> anyhow::Result<Self> {
        let p = p.as_ref();
        let meta = FileMeta::from_path(p)?;

        Ok(Self {
            path: p.to_path_buf(),
            meta,
        })
    }
}

pub struct FileIndex {
    files: BTreeMap<String, FileEntry>,
}

impl FileIndex {
    pub fn get_index(&self) -> Vec<FileMeta> {
        self.files.values().map(|f| f.meta.clone()).collect()
    }


    pub fn build_index<P: AsRef<Path>>(
        file_paths: impl Iterator<Item = P>,
    ) -> anyhow::Result<Self> {
        let mut files = BTreeMap::new();
        for file in file_paths {
            if !file.as_ref().exists() {
                continue;
            }
             
            let entry = FileEntry::from_path(file)?;
            files.insert(entry.meta.name.to_lowercase(), entry);
        }

        Ok(Self { files })
    }

    pub fn get(&self, name: &str) -> Option<&FileEntry> {
        let key = name.to_lowercase();
        self.files.get(&key)
    }

    pub fn get_updates<'a>(
        &'a self,
        remote_index: impl Iterator<Item = &'a FileMeta> + 'a,
    ) -> impl Iterator<Item = &'a FileMeta> + 'a {
        remote_index.filter(|meta| {
            self.get(&meta.name)
                .map(|m| m.meta.should_update(meta))
                .unwrap_or(true)
        })
    }
}

pub trait DownloadProgressWatcher {
    fn update(&self, rx: u64, total: u64);
}

pub struct FileClient {
    base: url::Url,
    client: reqwest::Client,
}

impl FileClient {
    pub fn new(base: url::Url) -> Self {
        Self {
            base,
            client: reqwest::ClientBuilder::new().build().unwrap(),
        }
    }

    pub fn from_url_str(base: &str) -> Self {
        Self::new(base.parse().expect("Must be valid URL"))
    }

    pub async fn get_index(&self) -> anyhow::Result<Vec<FileMeta>> {
        let url = self.base.join("index")?;
        Ok(self.client.get(url).send().await?.json().await?)
    }

    pub async fn download_file<W: DownloadProgressWatcher>(
        &self,
        meta: &FileMeta,
        dst: &mut tokio::fs::File,
        watcher: &W,
    ) -> anyhow::Result<()> {
        let url = self.base.join(&format!("file/{}", meta.name))?;

        let resp = self.client.get(url).send().await?;

        let len = resp
            .content_length()
            .ok_or_else(|| anyhow!("Response must have content length"))?;
        let mut bytes = resp.bytes_stream();
        let mut rx_bytes = 0;

        watcher.update(rx_bytes, len);

        while let Some(chunk) = bytes.next().await {
            let chunk = chunk?;
            dst.write_all(&chunk).await?;
            rx_bytes += chunk.len() as u64;
            watcher.update(rx_bytes, len);
        }

        Ok(())
    }

    pub async fn update_file<W: DownloadProgressWatcher>(
        &self,
        meta: &FileMeta,
        dst_path: impl AsRef<Path>,
        watcher: &W,
    ) -> anyhow::Result<()> {
        let p = dst_path.as_ref();
        let new = !p.exists();
        let dl_path = if new {
            p.to_path_buf()
        } else {
            p.with_extension(".new")
        };

        let mut file = tokio::fs::File::create(&dl_path).await?;
        self.download_file(meta, &mut file, watcher).await?;
        file.flush().await?;

        if !new {
            let old = p.to_path_buf().with_extension("old");
            // Rename file
            std::fs::rename(&dst_path, &old)?;
            std::fs::rename(dl_path, dst_path)?;

            // attempt to delete file, failure is no problem
            let _ = std::fs::remove_file(&old);
        }

        Ok(())
    }

    pub async fn update_files(
        &self,
        local_index: &FileIndex,
        watcher: impl DownloadProgressWatcher,
    ) -> anyhow::Result<()> {
        let remote_index = self.get_index().await?;

        for update in local_index.get_updates(remote_index.iter()) {
            self.update_file(update, update.name.clone(), &watcher).await?;
        }

        Ok(())
    }
}
