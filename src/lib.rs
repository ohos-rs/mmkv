mod sys;

use crate::sys::{MMKVLogLevel, MMKVMode, MMKV};
use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::BigInt;
use napi_ohos::{Either, JsBigInt};
use std::ffi::{c_char, c_int, CStr, CString};

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

    /// set number include int float etc.s
    #[napi]
    pub fn encode_number(&self, key: String, value: f64, _expire: Option<i32>) {
        let k = CString::new(key).unwrap();
        unsafe {
            sys::set_double(self.inner.clone(), value, k.as_ptr().cast());
        }
    }

    /// get number
    #[napi]
    pub fn decode_number(&self, key: String) -> f64 {
        let k = CString::new(key).unwrap();
        unsafe { sys::get_double(self.inner.clone(), k.as_ptr().cast()) as f64 }
    }

    /// set bigint which will store as `Vec<string>`, and the first element is a flag, 1 for negative numbers.
    #[napi]
    pub fn encode_bigint(&self, key: String, mut value: JsBigInt, _expire: Option<i32>) {
        let k = CString::new(key).unwrap();
        let (signed, mut words) = value.get_words().unwrap();
        words.insert(
            0,
            match signed {
                true => 1,
                false => 0,
            },
        );
        let c_strings = words
            .iter()
            .map(|&num| CString::new(num.to_string()).unwrap())
            .collect::<Vec<CString>>();
        let c_ptrs: Vec<*const c_char> = c_strings.iter().map(|s| s.as_ptr().cast()).collect();
        unsafe {
            sys::set_string_list(
                self.inner.clone(),
                c_ptrs.as_ptr().cast(),
                c_ptrs.len() as i32,
                k.as_ptr().cast(),
            );
        }
    }

    /// get bigint
    #[napi]
    pub fn decode_bigint(&self, key: String) -> BigInt {
        let k = CString::new(key).unwrap();
        let mut length: c_int = 0;
        let c_strings = unsafe {
            let ptr = sys::get_string_list(self.inner.clone(), &mut length, k.as_ptr().cast());
            std::slice::from_raw_parts(ptr, length as usize)
        };

        let mut strings: Vec<u64> = c_strings
            .iter()
            .map(|&c_str| {
                unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() }
                    .parse()
                    .unwrap()
            })
            .collect();
        let flags = match strings.remove(0) {
            0 => false,
            _ => true,
        };
        BigInt {
            words: strings,
            sign_bit: flags,
        }
    }

    /// remove key or keys
    #[napi]
    pub fn remove_value_for_key(&self, key: Either<String, Vec<String>>) {
        match key {
            Either::A(k) => {
                let remove_key = CString::new(k).unwrap();
                unsafe {
                    sys::remove_value_for_key(self.inner.clone(), remove_key.as_ptr().cast());
                }
            }
            Either::B(keys) => {
                let remove_keys: Vec<*const c_char> = keys
                    .iter()
                    .map(|v| {
                        let t = CString::new(v.as_str()).unwrap();
                        t.as_ptr().cast()
                    })
                    .collect::<Vec<*const c_char>>();
                unsafe {
                    sys::remove_values_for_keys(
                        self.inner.clone(),
                        remove_keys.as_ptr().cast(),
                        remove_keys.len() as i32,
                    );
                }
            }
        }
    }

    /// check key if existed
    #[napi]
    pub fn contains_key(&self, key: String) -> bool {
        let k = CString::new(key).unwrap();
        unsafe { sys::contains_key(self.inner.clone(), k.as_ptr().cast()) }
    }

    /// get current mmkv instance's all keys
    #[napi]
    pub fn all_keys(&self) -> Vec<String> {
        let mut length: c_int = 0;
        let c_strings = unsafe {
            let ptr = sys::all_keys(self.inner.clone(), &mut length);
            std::slice::from_raw_parts(ptr, length as usize)
        };

        let strings: Vec<String> = c_strings
            .iter()
            .map(|&c_str| {
                unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() }
                    .parse()
                    .unwrap()
            })
            .collect();

        strings
    }

    /// get current mmkv instance's mmap id
    #[napi]
    pub fn get_mmap_id(&self) -> String {
        unsafe {
            let c_value = sys::get_mmap_id(self.inner.clone());
            let r_value = CStr::from_ptr(c_value).to_str().unwrap();
            r_value.to_string()
        }
    }

    /// get current mmkv instance's storage size
    /// @default TOTAL
    #[napi]
    pub fn get_storage_size(
        &self,
        #[napi(ts_arg_type = "'ACTUAL' | 'TOTAL'")] size_type: Option<String>,
    ) -> i32 {
        match size_type {
            Some(t) => match t.as_str() {
                "ACTUAL" => unsafe { sys::get_actual_size(self.inner.clone()) },
                _ => unsafe { sys::get_total_size(self.inner.clone()) },
            },
            None => unsafe { sys::get_total_size(self.inner.clone()) },
        }
    }
}
