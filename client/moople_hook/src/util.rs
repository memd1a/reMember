use std::{ffi::OsString, path::PathBuf, ptr, os::windows::prelude::OsStringExt};

use detour::GenericDetour;
use region::Protection;
use windows::{
    core::{PCSTR, PCWSTR},
    Win32::{
        Foundation::MAX_PATH,
        System::{
            LibraryLoader::{GetModuleHandleW, GetProcAddress},
            SystemInformation::GetSystemDirectoryW,
        },
    },
};

extern "C" {
    #[link_name = "llvm.returnaddress"]
    pub fn return_address(a: i32) -> *const u8;
}

#[macro_export]
macro_rules! ret_addr {
    () => {
        unsafe { return_address(0) as usize }
    };
}

#[macro_export]
macro_rules! fn_ref {
    ($name:ident,$fn_name:ident, $addr_name:ident, $addr:expr, $($fn_ty:tt)*) => {
        #[allow(non_upper_case_globals)]
        pub const $addr_name: *const () = $addr as *const ();
        pub type $fn_name = $($fn_ty)*;
        #[allow(non_upper_case_globals)]
        pub static $name: std::sync::LazyLock<$fn_name> = std::sync::LazyLock::new(|| unsafe {
            std::mem::transmute($addr_name)
        });
    };
}

#[macro_export]
macro_rules! fn_ref2 {
    ($name:ident, $addr:expr, $($fn_ty:tt)*) => {
        paste::paste! {
            #[allow(non_upper_case_globals)]
            pub const [<$name _addr>]: *const () = $addr as *const ();
            pub type [<$name:camel>] = $($fn_ty)*;
            #[allow(non_upper_case_globals)]
            pub static $name: std::sync::LazyLock<[<$name:camel>]> = std::sync::LazyLock::new(|| unsafe {
                std::mem::transmute([<$name _addr>])
            });
        }
    };
}

#[macro_export]
macro_rules! fn_ref_hook {
    ($name:ident,$fn_name:ident, $addr_name:ident, $addr:expr, $hook_name:ident, $($fn_ty:tt)*) => {
        fn_ref!($name, $fn_name, $addr_name, $addr, $($fn_ty)*);
        detour::static_detour! {
            static $hook_name: $($fn_ty)*;
        }
    };
}

#[macro_export]
macro_rules! fn_ref_hook2 {
    ($name:ident, $addr:expr, $hook_name:ident, $($fn_ty:tt)*) => {
        fn_ref2!($name, $addr, $($fn_ty)*);
        detour::static_detour! {
            static $hook_name: $($fn_ty)*;
        }
    };
}

pub unsafe fn ms_fn_hook<F: detour::Function + Sized>(addr: usize, detour: F) -> GenericDetour<F> {
    let f: F = std::mem::transmute_copy(&addr);
    GenericDetour::new(f, detour).expect("MS detour")
}

//TODO impl hookable trait for unsafe fns
#[macro_export]
macro_rules! static_ms_fn_hook {
    ($name:ident, $addr:expr, $detour:ident, type $fnty:ident = $($fn_ty:tt)*) => {
        pub type $fnty = $($fn_ty)*;
        static $name: LazyLock<GenericDetour<$fnty>> =
            LazyLock::new(|| unsafe { $crate::util::ms_fn_hook::<$fnty>($addr, $detour) });
    };
}

pub unsafe fn win32_fn_hook<F: detour::Function + Sized>(
    module: PCWSTR,
    fn_name: PCSTR,
    detour: F,
) -> GenericDetour<F> {
    let handle = GetModuleHandleW(module).expect("Module");
    let proc = GetProcAddress(handle, fn_name);
    let Some(proc) = proc else {
        panic!("Unknown function {fn_name:?} for module: {module:?}");
    };

    let win_fn: F = std::mem::transmute_copy(&proc);
    GenericDetour::new(win_fn, detour).expect("Win32 detour")
}

#[macro_export]
macro_rules! static_win32_fn_hook {
    ($name:ident, $mod:expr, $fn_name:expr, $detour:ident, type $fnty:ident = $($fn_ty:tt)*) => {
        pub type $fnty = $($fn_ty)*;
        static $name: LazyLock<GenericDetour<$fnty>> =
            LazyLock::new(|| unsafe { $crate::util::win32_fn_hook::<$fnty>($mod, $fn_name, $detour) });
    };
}

pub unsafe fn ms_memset(mut addr: *mut u8, b: u8, cnt: usize) -> region::Result<()> {
    let _handle = region::protect_with_handle(addr, cnt, Protection::READ_WRITE_EXECUTE)?;

    for _ in 0..cnt {
        addr.write_volatile(b);
        addr = addr.offset(1);
    }

    Ok(())
}

pub unsafe fn ms_memcpy(addr: *mut u8, src: *const u8, cnt: usize) -> region::Result<()> {
    let _handle = region::protect_with_handle(addr, cnt, Protection::READ_WRITE_EXECUTE)?;

    ptr::copy(src, addr, cnt);
    Ok(())
}

pub unsafe fn nop(addr: *mut u8, cnt: usize) -> region::Result<()> {
    ms_memset(addr, 0x90, cnt)
}

pub fn get_sys_path() -> anyhow::Result<PathBuf> {
    let mut buf = [0; (MAX_PATH + 1) as usize];
    let n = unsafe { GetSystemDirectoryW(Some(&mut buf)) } as usize;
    if n == 0 {
        anyhow::bail!("Unable to get sys dir");
    }

    Ok(OsString::from_wide(&buf[..n]).into())
}
