#![allow(dead_code)]
use std::{fs::File, path::Path, process::Command, time::Duration};

use indicatif::ProgressBar;
use replacer::{Patch, Replacer};
use shrooming::{files::DownloadProgressWatcher, FileClient, FileIndex};

mod replacer;

/*
    Plan:
        * Provide an update api for the proxy dll
        * Write updating in a generic way maybe use bidiff crate for now only the launcher + proxy dll are required to be updated
        * Enable the launcher to unpack(for now) wz files into img files with a folder structure, later use tar or another proven archive format
        * Etablish a grpc communication to the server
        * Allow dev auto login bringing the client straight into the game
        * transmit crash/exception info


*/

fn patch_import(file: impl AsRef<Path>) -> anyhow::Result<()> {
    let patch = vec![Patch::Replace {
        needle: b"dinput8",
        replace: b"dinpuz8",
    }];
    let new_file = file.as_ref().to_path_buf().with_extension(".exe.patched");
    let r = File::open(file)?;
    let w = File::create(new_file)?;

    let mut patcher = Replacer::new(r, w, patch);
    patcher.run::<4096>()?;

    Ok(())
}

pub struct DownloadProgressWatcherBar(ProgressBar);

impl DownloadProgressWatcherBar {
    pub fn new() -> Self {
        let pb = ProgressBar::new(100);
        /*pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));*/
        Self(pb)
    }
}

impl DownloadProgressWatcher for DownloadProgressWatcherBar {
    fn update(&self, rx: u64, total: u64) {
        let perc = (rx * 100) / total;
        self.0.set_position(perc);
    }
}

fn launch_moople(addr: &str, port: u16) -> anyhow::Result<()> {
    Command::new("GMSv95_.exe")
        .arg(addr)
        .arg(port.to_string())
        .spawn()?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Moople Launchar v1.0");
    tokio::time::sleep(Duration::from_secs(1)).await;

    let updater = FileClient::from_url_str("http://192.168.124.1:8490");

    let progress = DownloadProgressWatcherBar::new();
    let local_ix = FileIndex::build_index(
        ["dinput8.dll", "moople_launchar.exe", "notes.txt"]
            .as_slice()
            .iter(),
    )?;

    updater.update_files(&local_ix, progress).await?;
    launch_moople("192.168.124.1", 8484)?;

    Ok(())
}
