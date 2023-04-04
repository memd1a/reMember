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

pub mod config;
pub mod packet_struct;
pub mod socket;
pub mod strings;
pub mod util;
pub mod wz_img;
pub mod ztl;

#[cfg(feature = "overlay")]
pub mod overlay;

use config::addr;
use detour::GenericDetour;
use log::LevelFilter;
use packet_struct::RECV_PACKET_CTX;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::ffi::c_void;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, LazyLock};
use std::time::Duration;
use windows::core::{IUnknown, GUID, HRESULT, HSTRING, PCSTR};
use windows::Win32::Foundation::{BOOL, HANDLE, HMODULE};
use windows::Win32::Security::SECURITY_ATTRIBUTES;
use windows::Win32::Storage::FileSystem::{FindFileHandle, WIN32_FIND_DATAA};
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};
use windows::{s, w};

use crate::config::addr::{AES_BASIC_KEY, AES_USER_KEY, IG_CIPHER_SEED, IG_SHUFFLE_KEY};
use crate::config::{DATA_DIR, DUMP_KEYS};
use crate::packet_struct::SEND_PACKET_CTX;
use crate::strings::StringPool;
use crate::util::{nop, return_address};

// Exception Handler hook to log exceptions
static_ms_fn_hook!(
    CXX_THROW_EXCEPTION_8_HOOK,
    addr::CXX_EXCEPTION,
    cxx_throw_exception_8_detour,
    type FCxxException = unsafe extern "cdecl" fn(*const c_void, *const c_void) -> u8
);
extern "cdecl" fn cxx_throw_exception_8_detour(
    ex_obj: *const c_void,
    throw_info: *const c_void,
) -> u8 {
    let ret = ret_addr!();
    log::error!("Exception at: {ret:X}");
    RECV_PACKET_CTX.finish_incomplete(0, ret);

    unsafe { CXX_THROW_EXCEPTION_8_HOOK.call(ex_obj, throw_info) }
}

static_win32_fn_hook!(
    FIND_FIRST_FILE_A_HOOK,
    w!("kernel32.dll"),
    s!("FindFirstFileA"),
    find_first_file_detour,
    type FnFindFirstFileA = extern "system" fn(PCSTR, *mut WIN32_FIND_DATAA) -> FindFileHandle
);

// Spoof the first call to FindFirstFileA, to hide this proxy DLL
extern "system" fn find_first_file_detour(
    file_name: PCSTR,
    find_file_data: *mut WIN32_FIND_DATAA,
) -> FindFileHandle {
    static SPOOFED_PROXY_DLL: AtomicBool = AtomicBool::new(false);
    if !file_name.is_null() && unsafe { file_name.as_bytes() } == b"*" {
        //Only spoof once at start
        if !SPOOFED_PROXY_DLL.fetch_or(true, Ordering::SeqCst) {
            log::info!("Spoofing FindFirstFileA for proxy dll");
            // Let it iterate over wz files
            return FIND_FIRST_FILE_A_HOOK.call(windows::s!("*.wz"), find_file_data);
        }
    }
    FIND_FIRST_FILE_A_HOOK.call(file_name, find_file_data)
}

// Multi client support by simply appending the process ID to each Mutex name
static_win32_fn_hook!(
    CREATE_MUTEX_A_HOOK,
    w!("kernel32.dll"),
    s!("CreateMutexA"),
    create_mutex_a_detour,
    type FnCreateMutexA = extern "system" fn(*const SECURITY_ATTRIBUTES, BOOL, PCSTR) -> HANDLE
);

extern "system" fn create_mutex_a_detour(
    lpmutexattributes: *const SECURITY_ATTRIBUTES,
    binitialowner: BOOL,
    name: PCSTR,
) -> HANDLE {
    let ret = ret_addr!();
    if !name.is_null() {
        log::info!("Spoofing CreateMutexA for multi client: {ret:X}");
        let name = std::str::from_utf8(unsafe { name.as_bytes() }).unwrap();
        // Add pid to the name to support multi client
        let pid = std::process::id();
        // Could be made nicer, when the windows crate has interpolation for the s! macro
        let new_name = format!("{name}_{pid}");
        let p_new_name = PCSTR::from_raw(new_name.as_ptr());

        return CREATE_MUTEX_A_HOOK.call(lpmutexattributes, binitialowner, p_new_name);
    }
    CREATE_MUTEX_A_HOOK.call(lpmutexattributes, binitialowner, name)
}

type FDirectInput8Create = unsafe extern "stdcall" fn(
    hinst: HMODULE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: IUnknown,
) -> HRESULT;

struct State {
    directinput8create: FDirectInput8Create,
}

static STATE: LazyLock<State> = LazyLock::new(|| unsafe {
    let sys_dir = util::get_sys_path().expect("Sysdir").join("dinput8.dll");
    let dinput8 = LoadLibraryW(&HSTRING::from(sys_dir.as_os_str())).expect("dinput8.dll");
    let directinput8create =
        GetProcAddress(dinput8, s!("DirectInput8Create")).expect("DirectInput8Create");
    State {
        directinput8create: std::mem::transmute(directinput8create),
    }
});

#[no_mangle]
unsafe extern "stdcall" fn DirectInput8Create(
    hinst: HMODULE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: IUnknown,
) -> HRESULT {
    (STATE.directinput8create)(hinst, dwversion, riidltf, ppvout, punkouter)
}

fn dump_str_pool() -> anyhow::Result<()> {
    let str_pool = StringPool::instance();
    str_pool.dump_ascii_string_pool(config::STR_POOL_FILE)?;
    str_pool.dump_utf16_string_pool(config::STR_POOL_UTF16_FILE)?;
    Ok(())
}

fn setup_logs() {
    unsafe { AllocConsole() };
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .ok();
}

fn init_hooks() -> anyhow::Result<()> {
    #[cfg(feature = "overlay")]
    overlay::init_hooks();

    unsafe {
        if config::PACKET_TRACING {
            socket::init_hooks()?;
        }

        CXX_THROW_EXCEPTION_8_HOOK.enable()?;
    }

    Ok(())
}

pub fn dump_keys() -> anyhow::Result<()> {
    fn dump_key(key_name: &str, addr: usize, len: usize) -> anyhow::Result<()> {
        let key = unsafe { std::slice::from_raw_parts(addr as *const u8, len) };

        let file = format!("{}/{key_name}.bin", DATA_DIR);
        let mut f = File::create(file)?;
        f.write_all(key)?;
        Ok(())
    }
    dump_key("aes_user_key", AES_USER_KEY, 4 * 32)?;
    dump_key("aes_basic_key", AES_BASIC_KEY, 32)?;
    dump_key("ig_seed", IG_CIPHER_SEED, 4)?;
    dump_key("ig_shuffle", IG_SHUFFLE_KEY, 0x100)?;

    Ok(())
}

fn exec() {
    let data_dir = Path::new(DATA_DIR);
    if !data_dir.exists() {
        std::fs::create_dir(data_dir).expect("Data dir");
    }

    if DUMP_KEYS {
        dump_keys().expect("Dump keys");
    }

    LazyLock::force(&SEND_PACKET_CTX);
    LazyLock::force(&RECV_PACKET_CTX);

    log::info!("Applying hooks and patches");
    if config::SKIP_LOGO {
        unsafe { nop(addr::LOGO_BRANCHES as *mut u8, 16).unwrap() };
    }
    init_hooks().unwrap();

    // Wait for the instances to be initialized, maybe should wait for the window to be created rather
    std::thread::sleep(Duration::from_secs(1));

    if config::DUMP_STR_POOL {
        if let Err(err) = dump_str_pool() {
            log::error!("Unable to dump string pool: {}", err);
        }
    }
}

unsafe fn stage_0_hooks() {
    FIND_FIRST_FILE_A_HOOK.enable().unwrap();
    CREATE_MUTEX_A_HOOK.enable().unwrap();
}

fn initialize() {
    setup_logs();
    log::info!("{} - {}", config::NAME, config::VERSION);

    // Load original dinput8
    LazyLock::force(&STATE);

    // Win32 Patches
    unsafe {
        stage_0_hooks();
    }

    std::thread::spawn(exec);
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HMODULE, call_reason: u32, reserved: *mut c_void) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            #[cfg(feature = "overlay")]
            overlay::init_module(dll_module);

            initialize();
        }
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    BOOL::from(true)
}
