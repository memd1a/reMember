#![feature(
    link_llvm_intrinsics,
    once_cell,
    abi_thiscall,
    pointer_byte_offsets,
    naked_functions,
    strict_provenance,
    asm_const
)]
#![recursion_limit = "512"]
// The whole library is unsafe no need to document the behaviour for now
#![allow(clippy::missing_safety_doc)]

pub const DATA_PATH: &str = "data";
pub const STRING_DATA_FILE: &str = "data/strings.json";
pub const PACKET_OUT_FILE: &str = "data/packet_out.json";
pub const PACKET_IN_FILE: &str = "data/packet_in.json";

pub mod wz_img;
pub mod packet_struct;
pub mod socket;
pub mod strings;
pub mod util;
pub mod ztl;

use detour::static_detour;
use packet_struct::RECV_PACKET_CTX;
use std::ffi::c_void;
use std::sync::LazyLock;
use std::time::Duration;

use strings::dump_string_pool;
use windows::core::{GUID, HRESULT};
use windows::s;
use windows::Win32::Foundation::{BOOL, HINSTANCE};
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

use crate::util::nop;

extern "C" {
    #[link_name = "llvm.returnaddress"]
    fn return_address(a: i32) -> *const u8;
}

fn_ref_hook!(
    cxx_throw_exception,
    CxxThrowException,
    cxx_throw_exception_addr,
    0x00a307a1,
    CxxThrowExceptionHook,
    unsafe extern "cdecl" fn(*const c_void, *const c_void) -> u8
);

fn cxx_throw_exception_8_detour(ex_obj: *const c_void, throw_info: *const c_void) -> u8 {
    let ret = unsafe { return_address(1) as usize };
    let ret2 = unsafe { return_address(2) as usize };
    log::info!("Exception @ {:X} - {:X}", ret, ret2);
    RECV_PACKET_CTX.finish_incomplete(0, ret);


    unsafe { CxxThrowExceptionHook.call(ex_obj, throw_info) }
}

type FDirectInput8Create = unsafe extern "stdcall" fn(
    hinst: HINSTANCE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: HINSTANCE,
) -> HRESULT;

struct State {
    directinput8create: FDirectInput8Create,
}

unsafe impl Send for State {}
unsafe impl Sync for State {}

static STATE: LazyLock<State> = LazyLock::new(|| unsafe {
    let dinput8 = LoadLibraryA(s!("C:\\Windows\\System32\\dinput8.dll")).unwrap();
    log::info!("Loaded dinput8 dll: {dinput8:?}");

    let directinput8create = GetProcAddress(dinput8, s!("DirectInput8Create"));
    log::info!("Found addr for create: {}", directinput8create.is_some());

    let directinput8create = std::mem::transmute(directinput8create);

    State { directinput8create }
});

fn initialize() {
    unsafe { AllocConsole() };

    // Patches
    
    // No logo
    unsafe { nop(0x60e2db as *mut u8, 16) };

    ::std::env::set_var("RUST_LOG", "DEBUG");
    //pretty_env_logger::init_custom_env("RUST_LOG=DEBUG");
    pretty_env_logger::init();

    println!("reMember - Moople Hook 1.1");
    log::info!("reMember - Moople Hook 1.1");
    LazyLock::force(&STATE);
}

fn init_hooks() -> anyhow::Result<()> {
    log::info!("Hooking...");

    unsafe {
        socket::init_hooks()?;

        CxxThrowExceptionHook
            .initialize(*cxx_throw_exception, cxx_throw_exception_8_detour)?
            .enable()?;
    }

    log::info!("Hooked");

    Ok(())
}


fn exec() {
    log::info!("Exec started");
    init_hooks().unwrap();
    std::thread::sleep(Duration::from_secs(1));
    log::info!("Dumping string pool...");
    dump_string_pool(STRING_DATA_FILE).unwrap();
}

#[no_mangle]
unsafe extern "stdcall" fn DirectInput8Create(
    hinst: HINSTANCE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: HINSTANCE,
) -> HRESULT {
    log::info!("Creating dinput8...");
    std::thread::spawn(exec);
    (STATE.directinput8create)(hinst, dwversion, riidltf, ppvout, punkouter)
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, reserved: *mut c_void) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => initialize(),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    BOOL::from(true)
}
