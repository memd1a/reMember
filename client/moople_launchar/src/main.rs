#![allow(dead_code)]

use std::{fs::File, path::Path};

use replacer::{Patch, Replacer};

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

fn main() {
    patch_import("Client.exe").unwrap()
}
