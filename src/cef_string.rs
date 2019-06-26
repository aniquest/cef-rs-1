use libc::{c_int, size_t};

#[cfg(target_os = "windows")]
#[link(name = "libcef", kind = "dylib")]
extern "stdcall" {
  fn cef_string_utf8_to_utf16(src: *const u8, src_len: size_t, cef_string: *mut CefString) -> c_int;
}
#[cfg(target_os = "linux")]
#[link(name = "cef", kind = "dylib")]
extern "C" {
  fn cef_string_utf8_to_utf16(src: *const u8, src_len: size_t, cef_string: *mut CefString) -> c_int;
}

#[repr(C)]
pub struct CefString { //UTF 8/16
  pub str_ptr: *const u16,
  pub length: usize,
  pub dtor:  extern fn(strPtr: c_int)
}

#[allow(unused_variables)]
extern fn nop_string(ptr: c_int) {
    return
}

impl CefString {
  pub fn empty() -> CefString {
    return CefString {
        str_ptr: std::ptr::null(),
        length: 0,
        dtor: nop_string
    }
  }

  pub fn from(string: &'static str) -> CefString {
    let mut cef_string  = CefString::empty();
    unsafe { cef_string_utf8_to_utf16(string.as_ptr(), string.len() as size_t, &mut cef_string); }
    return cef_string;
  }
}
