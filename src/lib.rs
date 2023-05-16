use std::{
    ffi::{c_char, CStr},
    ptr::NonNull,
};
mod c_str;
use c_str::COwnedString;

extern "C" {
    fn krcdecode(
        src: *mut ::std::os::raw::c_char,
        src_len: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    fn qrcdecode(
        src: *mut ::std::os::raw::c_char,
        src_len: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}

pub fn krc_decode(resp: &mut [c_char]) -> Option<COwnedString> {
    unsafe {
        NonNull::new(krcdecode(resp.as_mut_ptr(), resp.len() as _)).map(|ptr| {
            let len = CStr::from_ptr(ptr.as_ptr() as _).to_bytes().len() as isize;
            COwnedString::from_raw_parts(ptr, len)
        })
    }
}

pub fn qrc_decode(resp: &mut [c_char]) -> Option<COwnedString> {
    unsafe {
        NonNull::new(qrcdecode(resp.as_mut_ptr(), resp.len() as _)).map(|ptr| {
            let len = CStr::from_ptr(ptr.as_ptr() as _).to_bytes().len() as isize;
            COwnedString::from_raw_parts(ptr, len)
        })
    }
}
