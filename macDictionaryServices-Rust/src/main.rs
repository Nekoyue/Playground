#![allow(non_snake_case)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr::{null, null_mut};

use core_foundation::base::{CFAllocatorGetDefault, CFRange};
use core_foundation::string::{CFStringCreateWithCString, CFStringGetCString, CFStringGetLength, CFStringRef, kCFStringEncodingUTF8};

fn main() {
    println!("{}", define("ubiquitous").unwrap_or("null"));
    println!("{}", define("うなぎ").unwrap_or("null"));
    println!("{}", define("AnUndefinedWord").unwrap_or("null"));
}

fn define(word: &str) -> Option<&str> {
    unsafe {
        let cStr = CString::new(word).unwrap();
        let result: CFStringRef = DCSCopyTextDefinition(
            null_mut(),
            CFStringCreateWithCString(CFAllocatorGetDefault(), cStr.as_ptr(), kCFStringEncodingUTF8),
            CFRange { location: 0, length: word.chars().count() as isize });

        if result != null() {
            let bufferSize =
                CFStringGetLength(result) * 4; // func returns UTF-16 character counts, times 4 for non-ASCII buffer safety

            let mut c_chars: Vec<c_char> = Vec::with_capacity(bufferSize as usize);
            let buffer: *mut c_char = c_chars.as_mut_ptr();
            CFStringGetCString(result, buffer, bufferSize, kCFStringEncodingUTF8);

            return Option::Some(CStr::from_ptr(buffer).to_str().unwrap());
        }
    }
    return Option::None;
}

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    pub fn DCSCopyTextDefinition(
        dictionary: *mut c_void,
        textString: CFStringRef,
        range: CFRange,
    ) -> CFStringRef;
}