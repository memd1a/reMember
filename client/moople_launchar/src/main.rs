#![allow(dead_code)]

use std::{path::Path, fs::File};

use replacer::{Patch, Replacer};

mod replacer;

fn patch_import(file: impl AsRef<Path>) -> anyhow::Result<()> {
    let patch = vec![Patch::Replace { needle: b"dinput8", replace: b"dinpuz8" }];
    let new_file = file.as_ref().to_path_buf().with_extension(".exe.patched");
    let r = File::open(file)?;
    let w = File::create(new_file)?;

    let mut patcher = Replacer::new(r, w, patch);
    patcher.run::<4096>()?;


    Ok(())
}

fn main() {
    patch_import("Client.exe").unwrap()
}
