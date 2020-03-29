use libc::{size_t};
use std::os::raw::{c_int, c_char};
use cef_base;

#[repr(C)]
pub struct CefString { //UTF 8/16
  pub str_ptr: *mut u16,
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
        str_ptr: std::ptr::null_mut(),
        length: 0,
        dtor: nop_string
    }
  }

  pub fn from(string: &'static str) -> CefString {
    let mut cef_string = cef_sys::_cef_string_utf16_t::default();

    unsafe {
      cef_sys::cef_string_utf8_to_utf16(
        string.as_ptr() as *const c_char,
        string.len() as size_t,
        &mut cef_string
      );
    }

    return CefString::from_sys(&mut cef_string);
  }

  pub fn from_string(string: String) -> CefString {
    let mut cef_string = cef_sys::_cef_string_utf16_t::default();

    unsafe {
      cef_sys::cef_string_utf8_to_utf16(
        string.as_ptr() as *const c_char,
        string.len() as size_t,
        &mut cef_string
      );
    }

    return CefString::from_sys(&mut cef_string);
  }

  pub fn from_sys(sys: *const cef_sys::cef_string_t) -> CefString {
    unsafe {
      return CefString {
        str_ptr: (*sys).str,
        length: (*sys).length,
        dtor: nop_string
      };
    }
  }

  pub fn as_sys(&mut self) -> *mut cef_sys::cef_string_t {
    return unsafe {
      std::mem::transmute_copy(&self)
    }
    // return &mut cef_sys::_cef_string_utf16_t {
    //   str: self.str_ptr,
    //   length: self.length,
    //   dtor: None
    // }
  }

  pub fn as_str(&self) -> String {
    let mut cef_string_u8 = cef_sys::_cef_string_utf8_t::default();

    unsafe {
      cef_sys::cef_string_utf16_to_utf8(
        self.str_ptr,
        self.length,
        &mut cef_string_u8
      );
    }

    unsafe {
      return std::ffi::CString::from_raw(cef_string_u8.str).into_string().unwrap();
    }
  }
}

impl From<&mut CefString> for *mut cef_sys::_cef_string_utf16_t {
  fn from(rs: &mut CefString) -> Self {
    return cef_base::return_non_ref_counted_as_cef_sys(rs);
  }
}

impl From<&CefString> for *const cef_sys::_cef_string_utf16_t {
  fn from(rs: &CefString) -> Self {
    // return unsafe { std::mem::transmute(rs) }
    return cef_base::return_non_ref_counted_as_cef_sys(rs);
  }
}

impl From<CefString> for cef_sys::cef_string_utf16_t {
  fn from(mut rs: CefString) -> Self {
    // return unsafe { std::mem::transmute(rs) }
    // return cef_sys::_cef_string_utf16_t::default();
    return unsafe { *rs.as_sys() };
    // return unsafe { *cef_base::return_non_ref_counted_as_cef_sys(&CefString::from(&rs.as_str())) };
  }
}

// impl From<&CefString> for *const cef_sys::_cef_string_utf16_t {
//   fn from(rs: &CefString) -> Self {
//     return unsafe { std::mem::transmute_copy(&rs) }
//     // return cef_base::return_non_ref_counted_as_cef_sys(rs);
//   }
// }

pub type CefStringUserFree = *mut CefString;
