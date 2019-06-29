use cef_string::CefString;
use base::CefBase;
use libc::{c_void};

use std;

#[repr(C)]
struct ProcessMessage {
	base: CefBase,
	is_valid: extern fn(this: *mut ProcessMessage) -> i32,
	is_read_only: extern fn(this: *mut ProcessMessage) -> i32,
	copy_of: extern fn(this: *mut ProcessMessage) -> *mut ProcessMessage,
	get_name : extern fn(this: *mut ProcessMessage) -> *mut CefString, //Must be freed by caller
	get_argument_list: extern fn(this: *mut ProcessMessage) -> libc::c_int
}

#[repr(C)]
pub struct Client {
	base: CefBase,
  get_audio_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_context_menu_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_dialog_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_display_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_download_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_drag_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_find_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_focus_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_jsdialog_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_keyboard_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_life_span_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_load_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_render_handler: extern fn(this: *mut Client) -> *mut c_void,
	get_request_handler: extern fn(this: *mut Client) -> *mut c_void,
	on_process_message_received: extern fn(this: *mut Client, browser: libc::c_int, frame: *mut c_void, source_process: i32, message: *mut ProcessMessage) -> i32
}

#[allow(unused_variables)]
extern fn nop_handler (this: *mut Client) -> *mut c_void {
	return std::ptr::null_mut();
}

#[allow(unused_variables)]
extern fn nop_processor (this: *mut Client, browser: libc::c_int, frame: *mut c_void, source_process: i32, message: *mut ProcessMessage) -> i32 {
	return 0;
}

impl Client {
	pub fn default() -> Client {
		return Client {
			base: CefBase::get::<Client>(),
      get_audio_handler: nop_handler,
			get_context_menu_handler: nop_handler,
			get_dialog_handler: nop_handler,
			get_display_handler: nop_handler,
			get_download_handler: nop_handler,
			get_drag_handler: nop_handler,
			get_find_handler: nop_handler,
			get_focus_handler: nop_handler,
			get_jsdialog_handler: nop_handler,
			get_keyboard_handler: nop_handler,
			get_life_span_handler: nop_handler,
			get_load_handler: nop_handler,
			get_render_handler: nop_handler,
			get_request_handler: nop_handler,
			on_process_message_received: nop_processor
		}
	}
}

