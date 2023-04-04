use serde::Serialize;
use std::{ffi::c_void, fs::File, io::{Write, BufWriter}, path::Path};

use crate::{
    config::{self, addr},
    fn_ref,
    ztl::zxstr::{ZXString, ZXString16, ZXString8},
};

type PStringPool = *const c_void;

fn_ref!(
    string_pool_get_instance,
    FStringPoolGetInstance,
    get_inst_addr,
    addr::STRING_POOL_GET_INSTANCE,
    unsafe extern "cdecl" fn() -> PStringPool
);

fn_ref!(
    string_pool_get_string,
    FStringPoolString,
    get_str_addr,
    addr::STRING_POOL_GET_STR,
    unsafe extern "thiscall" fn(PStringPool, &mut ZXString8, u32) -> &ZXString8
);

fn_ref!(
    string_pool_get_string_w,
    FStringPoolStringW,
    get_str_addr_w,
    addr::STRING_POOL_GET_STRW,
    unsafe extern "thiscall" fn(PStringPool, &mut ZXString16, u32) -> &ZXString16
);

#[derive(Serialize)]
pub struct StringPoolEntry<'a> {
    id: usize,
    str: &'a str,
}

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

    pub fn dump_ascii_string_pool(&self, file: impl AsRef<Path>) -> anyhow::Result<()> {
        self.dump_string_pool(file, |pool, id| {
            pool.get_str(id)
                .map(|s| s.get_str().unwrap_or("Invalid utf8").to_string())
        })
    }

    pub fn dump_utf16_string_pool(&self, file: impl AsRef<Path>) -> anyhow::Result<()> {
        self.dump_string_pool(file, |pool, id| {
            pool.get_str_w(id).map(|s| s.get_str_owned())
        })
    }

    pub fn dump_string_pool(
        &self,
        str_file: impl AsRef<Path>,
        get_str: impl Fn(&Self, u32) -> Option<String>,
    ) -> anyhow::Result<()> {
        let p = str_file.as_ref();
        if p.exists() {
            log::info!(
                "Skipping dumping string pool already exists in file: {:?}",
                p
            );
            return Ok(());
        }

        let mut file = BufWriter::new(File::create(p)?);
        writeln!(file, "[")?;

        for index in 0..config::MAX_STR_POOL_LEN {
            if let Some(s) = get_str(self, index as u32) {
                serde_json::to_writer(
                    &mut file,
                    &StringPoolEntry {
                        id: index,
                        str: s.as_ref(),
                    },
                )?;
                writeln!(file, ",")?;
            }
        }

        writeln!(file, "]")?;

        log::info!("Dumped string pool: {p:?}");

        Ok(())
    }
}
