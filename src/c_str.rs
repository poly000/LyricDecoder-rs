use std::ops::Index;
use std::slice;
use std::{os::raw::c_char, ptr::NonNull};
use libc::free;

/// read-only C owned String
pub struct COwnedString {
    ptr: NonNull<c_char>,
    /// not contains the NULL termination
    len: isize,
}

impl COwnedString {
    pub unsafe fn from_raw_parts(ptr: NonNull<c_char>, len: isize) -> Self {
        if len.is_negative() {
            panic!("length of c-string should not be negative");
        }

        Self { ptr, len }
    }
}

impl Index<isize> for COwnedString {
    type Output = c_char;

    fn index(&self, index: isize) -> &Self::Output {
        let len = self.len;
        if index > len {
            panic!("index out of bounds: the len is {len} but the index is {index}");
        }

        unsafe { &*self.ptr.as_ptr().offset(index as _) }
    }
}

impl AsRef<[c_char]> for COwnedString {
    /// not containing the latest NULL
    fn as_ref(&self) -> &[c_char] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr() as _, self.len as _) }
    }
}

impl Drop for COwnedString {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr.as_ptr() as _);
        }
    }
}
