use napi_derive_ohos::napi;
use std::ffi::{c_char, c_double, c_int, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MMKV {
    _unused: [u8; 0],
}

/// MMKV log level
#[napi]
#[repr(C)]
pub enum MMKVLogLevel {
    Debug,
    Info,
    Warning,
    Error,
    None,
}

#[napi]
#[repr(C)]
pub enum MMKVMode {
    SingleProcess = 1,
    MultiProcess = 2,
}

type MMKVLogHandler = Option<
    unsafe extern "C" fn(
        log_level: MMKVLogLevel,
        file: *const c_char,
        line: c_int,
        function: *const c_char,
        message: *const c_char,
    ),
>;

extern "C" {
    pub fn get_mmkv_instance(
        mode: MMKVMode,
        crypt_key: *const c_char,
    ) -> *const MMKV;
    pub fn init_mmkv(
        root_dir: *const c_char,
        log_level: MMKVLogLevel,
        handler: MMKVLogHandler,
    ) -> c_void;
    pub fn set_bool(mmkv: *const MMKV, v: bool, k: *const c_char) -> c_void;
    pub fn get_bool(mmkv: *const MMKV, k: *const c_char) -> bool;
    pub fn set_i32(mmkv: *const MMKV, v: c_int, k: *const c_char) -> c_void;
    pub fn get_i32(mmkv: *const MMKV, k: *const c_char) -> c_int;
    pub fn set_f64(mmkv: *const MMKV, v: c_double, k: *const c_char) -> c_void;
    pub fn get_f64(mmkv: *const MMKV, k: *const c_char) -> c_double;
}