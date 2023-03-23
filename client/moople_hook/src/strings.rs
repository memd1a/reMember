use serde::Serialize;
use std::{ffi::c_void, fs::File, io::Write, path::Path};

use crate::{
    fn_ref,
    ztl::zxstr::{ZXString, ZXString16, ZXString8},
};

type PStringPool = *const c_void;

fn_ref!(
    string_pool_get_instance,
    FStringPoolGetInstance,
    get_inst_addr,
    0x007466a0,
    unsafe extern "cdecl" fn() -> PStringPool
);

fn_ref!(
    string_pool_get_string,
    FStringPoolString,
    get_str_addr,
    0x00403b30,
    unsafe extern "thiscall" fn(PStringPool, &mut ZXString8, u32) -> &ZXString8
);

fn_ref!(
    string_pool_get_string_w,
    FStringPoolStringW,
    get_str_addr_w,
    0x00403b60,
    unsafe extern "thiscall" fn(PStringPool, &mut ZXString16, u32) -> &ZXString16
);

pub struct StringPool(PStringPool);

impl StringPool {
    pub fn instance() -> Self {
        Self(unsafe { string_pool_get_instance() })
    }

    pub fn get_str(&self, id: u32) -> Option<ZXString8> {
        let mut zx_str = ZXString::empty();
        unsafe { string_pool_get_string(self.0, &mut zx_str, id) };
        zx_str.is_not_empty().then_some(zx_str)
    }

    pub fn get_str_w(&self, id: u32) -> Option<ZXString16> {
        let mut zx_str = ZXString::empty();
        unsafe { string_pool_get_string_w(self.0, &mut zx_str, id) };
        zx_str.is_not_empty().then_some(zx_str)
    }
}

#[derive(Serialize)]
pub struct StringPoolEntry<'a> {
    i: usize,
    str: &'a str,
}

pub fn dump_string_pool(str_file: impl AsRef<Path>) -> anyhow::Result<()> {
    let p = str_file.as_ref();
    if p.exists() {
        log::info!("Skipping dumping string pool already exists");
        return Ok(());
    }

    let string_pool = StringPool::instance();

    let mut file = File::create(p)?;
    writeln!(file, "[")?;
    for i in 0..6883 {
        if let Some(str) = string_pool.get_str(i as u32) {
            if i < 10 {
                dbg!(unsafe { str.get_str_data() });
            }
            let s = str.get_str().unwrap_or("NO DATA");

            let line = serde_json::to_string(&StringPoolEntry { i, str: s })?;
            writeln!(file, "{line},")?;
        }

        if let Some(str) = string_pool.get_str_w(i as u32) {
            let s = str.get_str_owned();
            let line = serde_json::to_string(&StringPoolEntry {
                i: i + 10_000,
                str: s.as_str(),
            })?;
            writeln!(file, "{line},")?;
        }
    }
    writeln!(file, "]")?;

    Ok(())
}
