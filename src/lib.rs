mod sys;

use crate::sys::{MMKVLogLevel, MMKVMode, MMKV};
use napi_derive_ohos::napi;
use std::ffi::{CStr, CString};

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

    #[napi]
    pub fn enable_auto_key_expire(&self, expire: u32) {
        unsafe {
            sys::enable_auto_key_expire(self.inner.clone(), expire);
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
        unsafe { sys::get_bool(self.inner.clone(), k.as_ptr().cast()) }
    }

    /// get string
    #[napi]
    pub fn decode_string(&self, key: String) -> String {
        let k = CString::new(key).unwrap();
        unsafe {
            let c_value = sys::get_string(self.inner.clone(), k.as_ptr().cast());
            let r_value = CStr::from_ptr(c_value).to_str().unwrap();
            r_value.to_string()
        }
    }

    /// set string
    #[napi]
    pub fn encode_string(&self, key: String, value: String, _expire: Option<i32>) {
        let k = CString::new(key).unwrap();
        let v = CString::new(value).unwrap();
        unsafe {
            sys::set_string(self.inner.clone(), v.as_ptr().cast(), k.as_ptr().cast());
        }
    }
}
