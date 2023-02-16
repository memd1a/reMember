use std::ffi::{c_void, c_short};

use crate::{fn_ref_hook, fn_ref};
use detour::static_detour;

pub type CWvsApp = c_void;
pub type IResMan = c_void;

//void __thiscall `CWvsApp::InitializeResMan`(class CWvsApp* `this`)
fn_ref_hook!(
    cwvs_app_init_res_man,
    CWvsAppInitializeResMan,
    cwvs_app_init_res_man_addr,
    0x009c9540,
    CWvsAppInitializeResManHook,
    unsafe extern "thiscall" fn(*const CWvsApp)
);

fn_ref!(
    pc_create_obj_iwz_res_man,
    PcCreateObjectIWzResMan,
    pc_create_obj_iwz_res_man_addr,
    0x9c2eb0,
    // sUOL, pObj, pUnkOuter
    unsafe extern "cdecl" fn(*const c_short, *const CWvsApp, *const c_void)
);