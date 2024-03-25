mod sys;

use crate::sys::{MMKVLogLevel, MMKVMode, MMKV};
use napi_derive_ohos::napi;
use std::ffi::CString;

#[napi(js_name = "MMKV")]
pub struct JsMMKV {
    inner: *const MMKV,
}

#[napi]
impl JsMMKV {
    #[napi(constructor)]
    pub fn new(root_dir: String, log_level: Option<MMKVLogLevel>, mode: Option<MMKVMode>) -> Self {
        let root_dir_c_str = CString::new(root_dir).unwrap();
        let level = log_level.unwrap_or(MMKVLogLevel::Info);
        unsafe {
            sys::init_mmkv(root_dir_c_str.as_ptr().cast(), level, None);
            JsMMKV {
                inner: sys::get_mmkv_instance(
                    mode.unwrap_or(MMKVMode::SingleProcess),
                    std::ptr::null(),
                ),
            }
        }
    }

    /// set bool to mmkv
    #[napi]
    pub fn encode_bool(&self, key: String, value: bool, _expire: Option<i32>) {
        let k = CString::new(key).unwrap();
        unsafe {
            sys::set_bool(self.inner.clone(), value, k.as_ptr().cast());
        }
    }

    /// get bool from mmkv
    #[napi]
    pub fn decode_bool(&self, key: String) -> bool {
        let k = CString::new(key).unwrap();
        unsafe {
            sys::get_bool(self.inner.clone(), k.as_ptr().cast())
        }
    }
}
