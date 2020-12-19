use std::ops::{Deref, DerefMut};
use std::os::raw::c_int;

pub struct Sequence {
    ptr: *mut c_int,
    size: usize,
}

impl Sequence {
    pub fn new(size: usize) -> Self {
        let ptr = unsafe { ffi_example_sys::get_sequential_array(size) };
        Self { ptr, size }
    }
}

impl Deref for Sequence {
    type Target = [c_int];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr, self.size) }
    }
}

impl DerefMut for Sequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size) }
    }
}

impl Drop for Sequence {
    fn drop(&mut self) {
        unsafe {
            ffi_example_sys::free_array(self.ptr);
        }
    }
}
