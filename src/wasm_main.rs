mod symbolic_base_bro;
mod convert;

/*
 * This file is published under the CC0 licence.
 *
 *      https://creativecommons.org/publicdomain/zero/1.0/deed.en
 *
 * Most of the code in this file has been copied from:
 *
 *      https://www.hellorust.com/demos/sha1/index.html
 */

use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};


// In order to work with the memory we expose (de)allocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

// The JavaScript side passes a pointer to a C-like string that's already placed into memory.
// On the Rust side we turn this into a CStr, extract the bytes, pass it through the crate
// and then turn it back into an memory-allocated C-like string.
// A pointer to this data is returned.
#[no_mangle]
pub extern "C" fn candidates(data: *mut c_char) -> *mut c_char {
    unsafe {
        let data = CStr::from_ptr(data).to_string_lossy().into_owned();
        // print everything and remove them in javascript, because handling those options via.
        // rust-javascript ffi is too much work.
        let options = convert::Options { 
            match_newlines: true, 
            print_equals:  true
        };
        let out = convert::string_by_candidates(&symbolic_base_bro::generate_candidates(&data), &options);
        let s = CString::new(out).unwrap();
        s.into_raw()
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

