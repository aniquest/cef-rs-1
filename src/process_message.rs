use std::os::raw::{c_int};
use cef_base;
use cef_string::{CefStringUserFree};
use cef_types::*;

pub type CefProcessId = i32;
pub const CEF_PROCESS_ID_PID_BROWSER: CefProcessId = 0;
pub const CEF_PROCESS_ID_PID_RENDERER: CefProcessId = 1;

pub const CEF_THREAD_ID_TID_UI: CefThreadId = 0;
pub const CEF_THREAD_ID_TID_FILE_BACKGROUND: CefThreadId = 1;
pub const CEF_THREAD_ID_TID_FILE: CefThreadId = 1;
pub const CEF_THREAD_ID_TID_FILE_USER_VISIBLE: CefThreadId = 2;
pub const CEF_THREAD_ID_TID_FILE_USER_BLOCKING: CefThreadId = 3;
pub const CEF_THREAD_ID_TID_PROCESS_LAUNCHER: CefThreadId = 4;
pub const CEF_THREAD_ID_TID_IO: CefThreadId = 5;
pub const CEF_THREAD_ID_TID_RENDERER: CefThreadId = 6;
pub type CefThreadId = i32;

#[repr(C)]
// #[derive(Debug, Default, Copy, Clone)]
pub struct CefProcessMessage {
    pub base: cef_base::CefBase,
    pub is_valid: extern "C" fn(self_: *mut CefProcessMessage) -> c_int,
    pub is_read_only: extern "C" fn(self_: *mut CefProcessMessage) -> c_int,
    pub copy: extern "C" fn(self_: *mut CefProcessMessage) -> *mut CefProcessMessage,
    pub get_name: extern "C" fn(self_: *mut CefProcessMessage) -> CefStringUserFree,
    pub get_argument_list: extern "C" fn(self_: *mut CefProcessMessage) -> *mut CefListValue
}

#[allow(unused_variables)]
pub extern "C" fn nop_processor (this: *mut CefProcessMessage) -> i32 {
  println!("CefProcessMessage::nop_processor called");
	return 0;
}

#[allow(unused_variables)]
pub extern "C" fn nop_copy (this: *mut CefProcessMessage) -> *mut CefProcessMessage {
  println!("CefProcessMessage::nop_copy called");
	return std::ptr::null_mut();
}

#[allow(unused_variables)]
pub extern "C" fn nop_get_name (this: *mut CefProcessMessage) -> CefStringUserFree {
  println!("CefProcessMessage::nop_get_name called");
	return std::ptr::null_mut();
}

#[allow(unused_variables)]
pub extern "C" fn nop_get_arg_list (this: *mut CefProcessMessage) -> *mut CefListValue {
  println!("CefProcessMessage::nop_get_arg_list called");
	return std::ptr::null_mut();
}

// #[cfg(target_os="windows")]
// #[link(name = "libcef", kind = "dylib")]
// extern "C" {
//   pub fn cef_process_message_create(name: *const CefString) -> *mut CefProcessMessage;
// }

impl CefProcessMessage {
  pub fn new(_message: &'static str) -> CefProcessMessage {
    // let name = CefString::from(message);

    // unsafe {
    //   let message = cef_process_message_create(&name);

      return CefProcessMessage {
        base: cef_base::CefBase::get::<CefProcessMessage>(),
        is_valid: nop_processor,
        is_read_only: nop_processor,
        copy: nop_copy,
        get_name: nop_get_name,
        get_argument_list: nop_get_arg_list
      }
    // }

    // let msg = cef_rs::process_message_create(CefString::from(text.as_str()));
  }
}
