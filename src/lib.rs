use std::{ffi::c_char, ptr::NonNull};

extern "C" {
    fn krcdecode(
        src: *mut ::std::os::raw::c_char,
        src_len: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    fn qrcdecode(
        src: *mut ::std::os::raw::c_char,
        src_len: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    fn free(_: *const ());
}

pub struct COwnedString {
    ptr: NonNull<c_char>,
}

impl Drop for COwnedString {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr.as_ptr() as _);
        }
    }
}

pub fn krc_decode(resp: &mut [c_char]) -> Option<COwnedString> {
    unsafe {
        NonNull::new(krcdecode(resp.as_mut_ptr(), resp.len() as _)).map(|ptr| COwnedString { ptr })
    }
}

pub fn qrc_decode(resp: &mut [c_char]) -> Option<COwnedString> {
    unsafe {
        NonNull::new(qrcdecode(resp.as_mut_ptr(), resp.len() as _)).map(|ptr| COwnedString { ptr })
    }
}
