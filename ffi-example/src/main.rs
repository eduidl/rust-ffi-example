use std::ffi::{CStr, CString};
use std::os::raw::c_char;

mod sequence;
use sequence::Sequence;

fn main() {
    let c_string = CString::new("Hello FFI").unwrap();
    unsafe {
        ffi_example_sys::print_str(c_string.as_ptr());
    }

    let c_string: CString = CString::new("Hello FFI").unwrap();
    let c_string_ptr: *const c_char = c_string.as_ptr();
    unsafe {
        ffi_example_sys::print_str(c_string_ptr);
    }

    let c_string_ptr = CString::new("Hello FFI").unwrap().as_ptr();
    unsafe {
        ffi_example_sys::print_str(c_string_ptr);
    }

    let str_ = unsafe { CStr::from_ptr(ffi_example_sys::hello()) }
        .to_str()
        .unwrap();
    println!("From Rust: {}", str_);

    let hello: *const c_char = unsafe { ffi_example_sys::hello() };
    let c_str: &CStr = unsafe { CStr::from_ptr(hello) };
    let str_: &str = c_str.to_str().unwrap();
    println!("From Rust: {}", str_);

    let slice = &[1, 13, 5];
    assert_eq!(
        unsafe { ffi_example_sys::sum(slice.as_ptr(), slice.len()) },
        19
    );

    let size = 101;
    let ptr = unsafe { ffi_example_sys::get_sequential_array(size) };
    let slice = unsafe { std::slice::from_raw_parts(ptr, size) };
    assert_eq!(slice.iter().fold(0, |sum, a| sum + a), 5050);
    drop(slice);
    unsafe {
        ffi_example_sys::free_array(ptr);
    }

    let ptr = unsafe { ffi_example_sys::get_sequential_array(size) };
    let cvec =
        unsafe { c_vec::CVec::new_with_dtor(ptr, size, |ptr| ffi_example_sys::free_array(ptr)) };
    assert_eq!(cvec.iter().fold(0, |sum, a| sum + a), 5050);

    let seq = Sequence::new(size);
    assert_eq!(seq.iter().fold(0, |sum, a| sum + a), 5050);
}
