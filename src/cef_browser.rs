use cef_string::{CefString, CefStringUserFree};
use process_message::{CefProcessId, CefProcessMessage};
use cef_base;
use libc::{c_void};
use cef_settings;
use platform::windows::CefWindowInfo;
use std::os::raw::{c_int};
use cef_types::*;

#[allow(unused_variables)]
pub extern fn nop_handler (this: *mut CefClient) -> *mut c_void {
	return std::ptr::null_mut();
}

#[allow(unused_variables)]
pub extern fn nop_processor (this: *mut CefClient, browser: libc::c_int, frame: *mut c_void, source_process: i32, message: *mut CefProcessMessage) -> i32 {
	return 0;
}

#[allow(unused_variables)]
pub extern fn on_message_received (this: *mut CefClient, browser: libc::c_int, frame: *mut c_void, source_process: i32, message: *mut CefProcessMessage) -> i32 {
  println!("on process message received");
	return 0;
}

#[repr(C)]
pub struct CefClient {
	base: cef_base::CefBase,
  get_audio_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_context_menu_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_dialog_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_display_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_download_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_drag_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_find_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_focus_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_jsdialog_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_keyboard_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_life_span_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_load_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_render_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	get_request_handler: extern fn(this: *mut CefClient) -> *mut c_void,
	on_process_message_received: extern fn(this: *mut CefClient, browser: libc::c_int, frame: *mut c_void, source_process: i32, message: *mut CefProcessMessage) -> i32
}

impl From<&mut CefClient> for *mut cef_sys::_cef_client_t {
  fn from(rs: &mut CefClient) -> Self {
    cef_base::add_ref_and_return_as_cef_sys(rs)
  }
}

impl CefClient {
	pub fn default() -> CefClient {
		return CefClient {
			base: cef_base::CefBase::get::<CefClient>(),
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
			on_process_message_received: on_message_received
		}
	}
}

///
// Class used to represent a frame in the browser window. When used in the
// browser process the methods of this class may be called on any thread unless
// otherwise indicated in the comments. When used in the render process the
// methods of this class may only be called on the main thread.
///
#[repr(C)]
pub struct CefFrame {
  pub base: cef_base::CefBase,
  pub is_valid: extern fn(self_: *mut CefFrame) -> c_int,
  pub undo: extern fn(self_: *mut CefFrame),
  pub redo: extern fn(self_: *mut CefFrame),
  pub cut: extern fn(self_: *mut CefFrame),
  pub copy: extern fn(self_: *mut CefFrame),
  pub paste: extern fn(self_: *mut CefFrame),
  pub del: extern fn(self_: *mut CefFrame),
  pub select_all: extern fn(self_: *mut CefFrame),
  pub view_source: extern fn(self_: *mut CefFrame),
  pub get_source: extern fn(self_: *mut CefFrame, visitor: *mut CefStringVisitor),
  pub get_text: extern fn(self_: *mut CefFrame, visitor: *mut CefStringVisitor),
  pub load_request: extern fn(self_: *mut CefFrame, request: *mut CefRequest),
  pub load_url: extern fn(self_: *mut CefFrame, url: *const CefString),
  pub load_string: extern fn(self_: *mut CefFrame, string_val: *const CefString, url: *const CefString),
  pub execute_java_script: extern fn(self_: *mut CefFrame, code: *const CefString, script_url: *const CefString, start_line: c_int),
  pub is_main: extern fn(self_: *mut CefFrame) -> c_int,
  pub is_focused: extern fn(self_: *mut CefFrame) -> c_int,
  pub get_name: extern fn(self_: *mut CefFrame) -> CefStringUserFree,
  pub get_identifier: extern fn(self_: *mut CefFrame) -> i64,
  pub get_parent: extern fn(self_: *mut CefFrame) -> *mut CefFrame,
  pub get_url: extern fn(self_: *mut CefFrame) -> CefStringUserFree,
  pub get_browser: extern fn(self_: *mut CefFrame) -> *mut CefBrowser,
  pub get_v8context: extern fn(self_: *mut CefFrame) -> *mut cef_sys::_cef_v8context_t,
  pub visit_dom: extern fn(self_: *mut CefFrame, visitor: *mut CefDomVisitor),
  pub create_urlrequest: extern fn(self_: *mut CefFrame, request: *mut CefRequest, client: *mut CefUrlRequestClient, ) -> *mut CefUrlRequest,
  pub send_process_message: extern fn(self_: *mut CefFrame, target_process: CefProcessId, message: *mut cef_sys::cef_process_message_t, ),
}

///
// Class used to represent a browser window. When used in the browser process
// the methods of this class may be called on any thread unless otherwise
// indicated in the comments. When used in the render process the methods of
// this class may only be called on the main thread.
///
#[repr(C)]
pub struct CefBrowser {
  pub base: cef_base::CefBase,
  pub get_host: extern fn(this: *mut CefBrowser) -> *mut CefBrowserHost,
  pub can_go_back: extern fn(this: *mut CefBrowser) -> c_int,
  pub go_back: extern fn(this: *mut CefBrowser),
  pub can_go_forward: extern fn(this: *mut CefBrowser) -> c_int,
  pub go_forward: extern fn(this: *mut CefBrowser),
  pub is_loading: extern fn(this: *mut CefBrowser) -> c_int,
  pub reload: extern fn(this: *mut CefBrowser),
  pub reload_ignore_cache: extern fn(this: *mut CefBrowser),
  pub stop_load: extern fn(this: *mut CefBrowser),
  pub get_identifier: extern fn(this: *mut CefBrowser) -> c_int,
  pub is_same: extern fn(this: *mut CefBrowser, that: *mut CefBrowser) -> c_int,
  pub is_popup: extern fn(this: *mut CefBrowser) -> c_int,
  pub has_document: extern fn(this: *mut CefBrowser) -> c_int,
  pub get_main_frame: extern fn(this: *mut CefBrowser) -> *mut CefFrame,
  pub get_focused_frame: extern fn(this: *mut CefBrowser) -> *mut CefFrame,
  pub get_frame_byident: extern fn(this: *mut CefBrowser, identifier: i64) -> *mut CefFrame,
  pub get_frame: extern fn(
          this: *mut CefBrowser,
          name: *const CefString,
      ) -> *mut CefFrame,
  pub get_frame_count: extern fn(this: *mut CefBrowser) -> usize,
  pub get_frame_identifiers: extern fn(this: *mut CefBrowser, identifiersCount: *mut usize, identifiers: *mut i64),
  pub get_frame_names: extern fn(this: *mut CefBrowser, names: ::std::os::raw::c_void/*cef_string_list_t*/),
}

impl CefBrowser {
  pub fn get_host(&mut self) -> *mut CefBrowserHost {
    // unsafe {
      (self.get_host)(self as *mut CefBrowser)
      // cef_sys::_cef_browser_t::get_host(self);
      // return (self as cef_sys::_cef_browser_t).get_host();
    // }
  }
	// pub fn default() -> Browser {
	// 	return Browser {
	// 		base: CefBase::get::<Browser>(),
  //     // get_host: unsafe { return self.get_host(); },//extern fn(this: *mut Browser) -> *mut BrowserHost,
  //     get_host: fn(this: *mut Browser) { 
  //       unsafe { cef_sys::_cef_browser_t::get_host }
  //     },
  //         // can_go_back: extern fn(this: *mut Browser) -> c_int,
  //     // go_back: extern fn(this: *mut Browser),
  //     // can_go_forward: extern fn(this: *mut Browser) -> c_int,
  //     // go_forward: extern fn(this: *mut Browser),
  //     // is_loading: extern fn(this: *mut Browser) -> c_int,
  //     // reload: extern fn(this: *mut Browser),
  //     // reload_ignore_cache: extern fn(this: *mut Browser),
  //     // stop_load: extern fn(this: *mut Browser),
  //     // get_identifier: extern fn(this: *mut Browser) -> c_int,
  //     // is_same: extern fn(this: *mut Browser, that: *mut Browser) -> c_int,
  //     // is_popup: extern fn(this: *mut Browser) -> c_int,
  //     // has_document: extern fn(this: *mut Browser) -> c_int,
  //     // get_main_frame: extern fn(this: *mut Browser) -> *mut Frame,
  //     // get_focused_frame: extern fn(this: *mut Browser) -> *mut Frame,
  //     // get_frame_byident: extern fn(this: *mut Browser, identifier: i64) -> *mut Frame,
  //     // get_frame: extern fn(
  //     //         this: *mut Browser,
  //     //         name: *const CefString,
  //     //     ) -> *mut Frame,
  //     // get_frame_count: extern fn(this: *mut Browser) -> usize,
  //     // get_frame_identifiers: extern fn(this: *mut Browser, identifiersCount: *mut usize, identifiers: *mut i64),
  //     // get_frame_names: extern fn(this: *mut Browser, names: ::std::os::raw::c_void/*cef_string_list_t*/),
	// 	}
	// }
}

#[repr(C)]
pub struct CefPoint {
  pub x: c_int,
  pub y: c_int,
}

#[repr(C)]
pub struct CefMouseEvent {
    pub x: i32,
    pub y: i32,
    pub modifiers: u32,
}

#[repr(C)]
pub struct CefRange {
    pub from: i32,
    pub to: i32,
}

#[repr(C)]
pub struct CefSize {
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
pub struct CefStringVisitor {
  pub base: cef_base::CefBase,
  pub visit: extern fn(self_: *mut CefStringVisitor, string: *const CefString),
}

#[repr(C)]
// #[derive(Debug, Default, Copy, Clone)]
pub struct CefRequest {
    pub base: cef_base::CefBase,
    pub is_read_only: extern fn(self_: *mut CefRequest) -> c_int,
    pub get_url: extern fn(self_: *mut CefRequest) -> CefStringUserFree,
    pub set_url: extern fn(self_: *mut CefRequest, url: *const CefString),
    pub get_method: extern fn(self_: *mut CefRequest) -> CefStringUserFree,
    pub set_method: extern fn(self_: *mut CefRequest, method: *const CefString),
    pub set_referrer: extern fn(
            self_: *mut CefRequest,
            referrer_url: *const CefString,
            policy: CefReferrerPolicy,
        ),
    pub get_referrer_url: extern fn(self_: *mut CefRequest) -> CefStringUserFree,
    pub get_referrer_policy: extern fn(self_: *mut CefRequest) -> CefReferrerPolicy,
    pub get_post_data: extern fn(self_: *mut CefRequest) -> *mut CefPostData,
    pub set_post_data: extern fn(self_: *mut CefRequest, postData: *mut CefPostData),
    pub get_header_map: extern fn(self_: *mut CefRequest, headerMap: CefStringMultimap),
    pub set_header_map: extern fn(self_: *mut CefRequest, headerMap: CefStringMultimap),
    pub get_header_by_name: extern fn(
            self_: *mut CefRequest,
            name: *const CefString,
        ) -> CefStringUserFree,
    pub set_header_by_name: extern fn(
            self_: *mut CefRequest,
            name: *const CefString,
            value: *const CefString,
            overwrite: c_int,
        ),
    pub set: extern fn(
            self_: *mut CefRequest,
            url: *const CefString,
            method: *const CefString,
            postData: *mut CefPostData,
            headerMapCefStringMultimap: CefStringMultimap,
        ),
    pub get_flags: extern fn(self_: *mut CefRequest) -> c_int,
    pub set_flags: extern fn(self_: *mut CefRequest, flags: c_int),
    pub get_first_party_for_cookies: extern fn(self_: *mut CefRequest) -> CefStringUserFree,
    pub set_first_party_for_cookies: extern fn(self_: *mut CefRequest, url: *const CefString),
    pub get_resource_type: extern fn(self_: *mut CefRequest) -> CefResourceType,
    pub get_transition_type: extern fn(self_: *mut CefRequest) -> c_void/*cef_transition_type_t*/,
    pub get_identifier:
        ::std::option::Option<unsafe extern "C" fn(self_: *mut CefRequest) -> u64>,
}

#[repr(C)]
pub struct CefBrowserHost {
  sys: cef_sys::_cef_browser_host_t,
}

impl From<&mut CefBrowserHost> for *mut cef_sys::_cef_browser_host_t {
  fn from(rs: &mut CefBrowserHost) -> Self {
    cef_base::add_ref_and_return_as_cef_sys(rs)
  }
}

impl CefBrowserHost {
  pub fn show_dev_tools(
    &mut self,
    window_info: &CefWindowInfo,
    client: &mut CefClient,
    settings: &cef_settings::CefBrowserSettings
  ) {
    unsafe {
      (self.sys.show_dev_tools.unwrap())(
        &mut self.sys,
        window_info.into(),
        client.into(),
        settings.into(),
        &mut cef_sys::_cef_point_t::default()
      );
    }
  }
}
