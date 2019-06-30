use cef_string::CefString;
use base::CefBase;
use libc::{c_void};
use settings::{CefState, BrowserSettings};
use platform::windows::WindowInfo;
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

#[repr(C)]
pub struct Frame {
  pub base: CefBase,
  // TODO add methods
}

#[repr(C)]
pub struct Browser {
  pub base: CefBase,
  pub get_host: extern fn(this: *mut Browser) -> *mut BrowserHost,
  pub can_go_back: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  pub go_back: extern fn(this: *mut Browser),
  pub can_go_forward: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  pub go_forward: extern fn(this: *mut Browser),
  pub is_loading: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  pub reload: extern fn(this: *mut Browser),
  pub reload_ignore_cache: extern fn(this: *mut Browser),
  pub stop_load: extern fn(this: *mut Browser),
  pub get_identifier: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  pub is_same: extern fn(this: *mut Browser, that: *mut Browser) -> ::std::os::raw::c_int,
  pub is_popup: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  pub has_document: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  pub get_main_frame: extern fn(this: *mut Browser) -> *mut Frame,
  pub get_focused_frame: extern fn(this: *mut Browser) -> *mut Frame,
  pub get_frame_byident: extern fn(this: *mut Browser, identifier: i64) -> *mut Frame,
  pub get_frame: extern fn(
          this: *mut Browser,
          name: *const CefString,
      ) -> *mut Frame,
  pub get_frame_count: extern fn(this: *mut Browser) -> usize,
  pub get_frame_identifiers: extern fn(this: *mut Browser, identifiersCount: *mut usize, identifiers: *mut i64),
  pub get_frame_names: extern fn(this: *mut Browser, names: ::std::os::raw::c_void/*cef_string_list_t*/),
}

impl Browser {
  pub fn get_host(&mut self) -> *mut BrowserHost {
    // unsafe {
      (self.get_host)(self as *mut Browser)
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
  //         // can_go_back: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  //     // go_back: extern fn(this: *mut Browser),
  //     // can_go_forward: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  //     // go_forward: extern fn(this: *mut Browser),
  //     // is_loading: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  //     // reload: extern fn(this: *mut Browser),
  //     // reload_ignore_cache: extern fn(this: *mut Browser),
  //     // stop_load: extern fn(this: *mut Browser),
  //     // get_identifier: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  //     // is_same: extern fn(this: *mut Browser, that: *mut Browser) -> ::std::os::raw::c_int,
  //     // is_popup: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
  //     // has_document: extern fn(this: *mut Browser) -> ::std::os::raw::c_int,
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
pub struct PdfPrintSettings {
  // TODO add methods
}

#[repr(C)]
pub struct CefPoint {
  pub x: ::std::os::raw::c_int,
  pub y: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct NavigationEntryVisitor {
  // TODO add methods
}

#[repr(C)]
pub struct CefKeyEvent {
  // TODO add methods
}

#[repr(C)]
pub struct CefMouseEvent {
    pub x: i32,
    pub y: i32,
    pub modifiers: u32,
}

#[repr(C)]
pub struct CefTouchEvent {
  // TODO add methods
}

#[repr(C)]
pub struct CefCompositionUnderline {
  // TODO add methods
}

#[repr(C)]
pub struct CefRange {
    pub from: i32,
    pub to: i32,
}

#[repr(C)]
pub struct CefDragData {
  // TODO add methods
}

#[repr(C)]
pub struct CefSize {
    pub width: i32,
    pub height: i32,
}

#[repr(C)]
pub struct CefNavigationEntry {
  // TODO add methods
}

#[repr(C)]
pub struct CefExtension {

}

#[repr(C)]
pub struct BrowserHost {
  pub base: CefBase,
  pub get_browser: extern fn(this: *mut BrowserHost) -> *mut Browser,
  pub close_browser: extern fn(this: *mut BrowserHost, force_close: ::std::os::raw::c_int),
  pub try_close_browser: extern fn(this: *mut BrowserHost) -> ::std::os::raw::c_int,
  pub set_focus: extern fn(this: *mut BrowserHost, focus: ::std::os::raw::c_int),
  pub get_window_handle: extern fn(this: *mut BrowserHost) -> u32,
  pub get_opener_window_handle: extern fn(this: *mut BrowserHost) -> u32,
  pub has_view: extern fn(this: *mut BrowserHost) -> ::std::os::raw::c_int,
  pub get_client: extern fn(this: *mut BrowserHost) -> *mut Client,
  pub get_request_context: extern fn(this: *mut BrowserHost) -> *mut c_void/*_cef_request_context_t*/,
  pub get_zoom_level: extern fn(this: *mut BrowserHost) -> f64,
  pub set_zoom_level: extern fn(this: *mut BrowserHost, zoomLevel: f64),
  pub run_file_dialog: extern fn(
      this: *mut BrowserHost,
      mode: i32/*cef_file_dialog_mode_t*/,
      title: *const CefString,
      default_file_path: *const CefString,
      accept_filters: ::std::os::raw::c_void/*cef_string_list_t*/,
      selected_accept_filter: ::std::os::raw::c_int,
      callback: *mut c_void/*_cef_run_file_dialog_callback_t*/,
    ),
  pub start_download: extern fn(this: *mut BrowserHost, url: *const CefString),
  pub download_image: extern fn(
      this: *mut BrowserHost,
      image_url: *const CefString,
      is_favicon: ::std::os::raw::c_int,
      max_image_size: u32,
      bypass_cache: ::std::os::raw::c_int,
      callback: *mut c_void/*_cef_download_image_callback_t*/,
    ),
  pub print: extern fn(this: *mut BrowserHost),
  pub print_to_pdf: extern fn(
      this: *mut BrowserHost,
      path: *const CefString,
      settings: *const c_void/*PdfPrintSettings*/,
      callback: *mut c_void/*_cef_pdf_print_callback_t*/,
    ),
  pub find: extern fn(
      this: *mut BrowserHost,
      identifier: ::std::os::raw::c_int,
      searchText: *const CefString,
      forward: ::std::os::raw::c_int,
      matchCase: ::std::os::raw::c_int,
      findNext: ::std::os::raw::c_int,
    ),
  pub stop_finding: extern fn(
      this: *mut BrowserHost,
      clearSelection: ::std::os::raw::c_int,
    ),
  pub show_dev_tools: extern fn(
      this: *mut BrowserHost,
      windowInfo: *const WindowInfo,
      client: *mut Client,
      settings: *const BrowserSettings,
      inspect_element_at: *const CefPoint,
    ),
  pub close_dev_tools: extern fn(this: *mut BrowserHost),
  pub has_dev_tools: extern fn(this: *mut BrowserHost) -> ::std::os::raw::c_int,
  pub get_navigation_entries: extern fn(
      this: *mut BrowserHost,
      visitor: *mut NavigationEntryVisitor,
      current_only: ::std::os::raw::c_int,
    ),
  pub set_mouse_cursor_change_disabled: extern fn(this: *mut BrowserHost, disabled: ::std::os::raw::c_int),
  pub is_mouse_cursor_change_disabled: extern fn(this: *mut BrowserHost) -> ::std::os::raw::c_int,
  pub replace_misspelling: extern fn(this: *mut BrowserHost, word: *const CefString),
  pub add_word_to_dictionary: extern fn(this: *mut BrowserHost, word: *const CefString),
  pub is_window_rendering_disabled: extern fn(this: *mut BrowserHost) -> ::std::os::raw::c_int,
  pub was_resized: extern fn(this: *mut BrowserHost),
  pub was_hidden: extern fn(this: *mut BrowserHost, hidden: ::std::os::raw::c_int),
  pub notify_screen_info_changed: extern fn(this: *mut BrowserHost),
  pub invalidate: extern fn(this: *mut BrowserHost, type_: i32/*cef_paint_element_type_t*/),
  pub send_external_begin_frame: extern fn(this: *mut BrowserHost),
  pub send_key_event: extern fn(this: *mut BrowserHost, event: *const CefKeyEvent),
  pub send_mouse_click_event: extern fn(
      this: *mut BrowserHost,
      event: *const CefMouseEvent,
      type_: i32/*cef_mouse_button_type_t*/,
      mouseUp: ::std::os::raw::c_int,
      clickCount: ::std::os::raw::c_int,
    ),
  pub send_mouse_move_event: extern fn(
      this: *mut BrowserHost,
      event: *const CefMouseEvent,
      mouseLeave: ::std::os::raw::c_int,
    ),
  pub send_mouse_wheel_event: extern fn(
      this: *mut BrowserHost,
      event: *const CefMouseEvent,
      deltaX: ::std::os::raw::c_int,
      deltaY: ::std::os::raw::c_int,
    ),
  pub send_touch_event: extern fn(this: *mut BrowserHost, event: *const CefTouchEvent),
  pub send_focus_event: extern fn(this: *mut BrowserHost, setFocus: ::std::os::raw::c_int),
  pub send_capture_lost_event: extern fn(this: *mut BrowserHost),
  pub notify_move_or_resize_started: extern fn(this: *mut BrowserHost),
  pub get_windowless_frame_rate: extern fn(this: *mut BrowserHost) -> ::std::os::raw::c_int,
  pub set_windowless_frame_rate: extern fn(this: *mut BrowserHost, frame_rate: ::std::os::raw::c_int),
  pub ime_set_composition: extern fn(
      this: *mut BrowserHost,
      text: *const CefString,
      underlinesCount: usize,
      underlines: *const CefCompositionUnderline,
      replacement_range: *const CefRange,
      selection_range: *const CefRange,
    ),
  pub ime_commit_text: extern fn(
      this: *mut BrowserHost,
      text: *const CefString,
      replacement_range: *const CefRange,
      relative_cursor_pos: ::std::os::raw::c_int,
    ),
  pub ime_finish_composing_text: extern fn(
        this: *mut BrowserHost,
        keep_selection: ::std::os::raw::c_int,
    ),
  pub ime_cancel_composition: extern fn(this: *mut BrowserHost),
  pub drag_target_drag_enter: extern fn(
      this: *mut BrowserHost,
      drag_data: *mut CefDragData,
      event: *const CefMouseEvent,
      allowed_ops: i32/*cef_drag_operations_mask_t*/,
    ),
  pub drag_target_drag_over: extern fn(
      this: *mut BrowserHost,
      event: *const CefMouseEvent,
      allowed_ops: i32,
    ),
  pub drag_target_drag_leave: extern fn(this: *mut BrowserHost),
  pub drag_target_drop: extern fn(this: *mut BrowserHost, event: *const CefMouseEvent),
  pub drag_source_ended_at: extern fn(
      this: *mut BrowserHost,
      x: ::std::os::raw::c_int,
      y: ::std::os::raw::c_int,
      op: i32,
    ),
  pub drag_source_system_drag_ended: extern fn(this: *mut BrowserHost),
  pub get_visible_navigation_entry: extern fn(this: *mut BrowserHost) -> *mut CefNavigationEntry,
  pub set_accessibility_state: extern fn(this: *mut BrowserHost, accessibility_state: CefState),
  pub set_auto_resize_enabled: extern fn(
      this: *mut BrowserHost,
      enabled: ::std::os::raw::c_int,
      min_size: *const CefSize,
      max_size: *const CefSize,
    ),
  pub get_extension: extern fn(this: *mut BrowserHost) -> *mut CefExtension,
  pub is_background_host: extern fn(this: *mut BrowserHost) -> ::std::os::raw::c_int,
  pub set_audio_muted: extern fn(this: *mut BrowserHost, mute: ::std::os::raw::c_int),
  pub is_audio_muted: extern fn(this: *mut BrowserHost) -> ::std::os::raw::c_int,
}

impl BrowserHost {
  pub fn show_dev_tools(
    &mut self,
    window_info: *const WindowInfo,
    client: *mut Client,
    settings: *const BrowserSettings,
    inspect_element_at: *const CefPoint
  ) {
      (self.show_dev_tools)(self as *mut BrowserHost, window_info, client, settings, inspect_element_at);
  }
}
