use napi_derive_ohos::napi;
use std::ffi::{c_char, c_double, c_int, c_uint, c_void};

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
    pub fn get_mmkv_instance(mode: MMKVMode, crypt_key: *const c_char) -> *const MMKV;
    pub fn get_mmkv_instance_with_id(
        id: *const c_char,
        mode: MMKVMode,
        crypt_key: *const c_char,
    ) -> *const MMKV;
    pub fn get_mmap_id(mmkv: *const MMKV) -> *const c_char;
    pub fn init_mmkv(
        root_dir: *const c_char,
        log_level: MMKVLogLevel,
        handler: MMKVLogHandler,
    ) -> c_void;
    pub fn set_bool(mmkv: *const MMKV, v: bool, k: *const c_char) -> c_void;
    pub fn get_bool(mmkv: *const MMKV, k: *const c_char) -> bool;
    pub fn get_string(mmkv: *const MMKV, k: *const c_char) -> *const c_char;
    pub fn set_string(mmkv: *const MMKV, v: *const c_char, k: *const c_char) -> c_void;
    pub fn enable_auto_key_expire(mmkv: *const MMKV, expire: c_uint) -> c_void;
    pub fn set_double(mmkv: *const MMKV, v: c_double, k: *const c_char) -> c_void;
    pub fn get_double(mmkv: *const MMKV, k: *const c_char) -> c_double;
    pub fn set_string_list(
        mmkv: *const MMKV,
        v: *const *const c_char,
        length: c_int,
        k: *const c_char,
    ) -> c_void;
    pub fn get_string_list(
        mmkv: *const MMKV,
        length: *mut c_int,
        k: *const c_char,
    ) -> *const *const c_char;
    pub fn remove_value_for_key(mmkv: *const MMKV, v: *const c_char) -> c_void;
    pub fn remove_values_for_keys(
        mmkv: *const MMKV,
        v: *const *const c_char,
        length: c_int,
    ) -> c_void;
    pub fn contains_key(mmkv: *const MMKV, v: *const c_char) -> bool;
    pub fn all_keys(mmkv: *const MMKV, length: *mut c_int) -> *const *const c_char;
    pub fn get_actual_size(mmkv: *const MMKV) -> c_int;
    pub fn get_total_size(mmkv: *const MMKV) -> c_int;
    pub fn count(mmkv: *const MMKV, filter_expire: bool) -> c_int;
    pub fn clear_all(mmkv: *const MMKV, keep_space: bool) -> c_void;
    pub fn clear_memory_cache(mmkv: *const MMKV, keep_space: bool) -> c_void;
    pub fn get_value_size(mmkv: *const MMKV, key: *const c_char, actual: bool) -> c_int;
    pub fn back_up(dir: *const c_char, mmap_id: *const c_char) -> c_int;
    pub fn restore(dir: *const c_char, mmap_id: *const c_char) -> c_int;
    pub fn remove_storage(mmap_id: *const c_char) -> c_void;
}
