use std::ptr;

use region::Protection;

#[macro_export]
macro_rules! fn_ref {
    ($name:ident,$fn_name:ident, $addr_name:ident, $addr:tt, $($fn_ty:tt)*) => {
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
    ($name:ident, $addr:tt, $($fn_ty:tt)*) => {
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
    ($name:ident,$fn_name:ident, $addr_name:ident, $addr:tt, $hook_name:ident, $($fn_ty:tt)*) => {
        fn_ref!($name, $fn_name, $addr_name, $addr, $($fn_ty)*);
        detour::static_detour! {
            static $hook_name: $($fn_ty)*;
        }
    };
}

#[macro_export]
macro_rules! fn_ref_hook2 {
    ($name:ident, $addr:tt, $hook_name:ident, $($fn_ty:tt)*) => {
        fn_ref2!($name, $addr, $($fn_ty)*);
        detour::static_detour! {
            static $hook_name: $($fn_ty)*;
        }
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