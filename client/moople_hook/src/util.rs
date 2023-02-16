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
    /* 
    ($name:ident,$fn_name:ident,$fn_ty:ty, $addr:tt) => {
        fn_ref!($name, $fn_name, $fn_ty, concat_idents!($name, _addr), $addr);
    }*/
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


pub unsafe fn nop(mut addr: *mut u8, cnt: usize) {
    region::protect(addr, cnt, Protection::READ_WRITE_EXECUTE).unwrap();

    for _ in 0..cnt {
        addr.write_volatile(0x90);
        addr = addr.offset(1);
    }
}