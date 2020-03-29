#![crate_type = "lib"]

pub mod cef_string;
pub mod cef_base;
pub mod cef_app;
pub mod cef_browser;
pub mod platform;
pub mod cef_settings;
pub mod process_message;
pub mod cef_types;
pub mod cef_v8;

extern crate cef_sys;
extern crate libc;
use libc::{c_int, c_void};

use platform::windows::{CefWindowInfo, MainArgs};

#[cfg(target_os="windows")]
#[link(name = "libcef", kind = "dylib")]
extern "C" {
// extern "stdcall" { //On windows32 stdcall. on 64 use C?
	fn cef_browser_host_create_browser(window_info: *const CefWindowInfo, client: *mut cef_browser::CefClient, url: *const cef_string::CefString, browser_settings: *const cef_sys::_cef_browser_settings_t, extra_info: *mut c_void, request_context: *mut c_void) -> c_int;
	// fn cef_browser_host_create_browser_sync(window_info: *const WindowInfo, client: *mut cef_browser::CefClient, url: *const cef_string::CefString, browser_settings: *const cef_sys::_cef_browser_settings_t, extra_info: *mut c_void, request_context: *mut c_void) -> *mut cef_browser::CefBrowser;
	fn cef_run_message_loop();
  fn cef_do_message_loop_work();
	fn cef_shutdown();
  // fn cef_process_message_create(name: *const CefString) -> *mut CefProcessMessage;
}
// #[cfg(target_os="linux")]
// #[link(name="cef")]
// extern "C" {
// 	fn cef_execute_process(args: *const MainArgs, app: *mut App, win_sandbox_info: *mut c_int) -> c_int;
// 	fn cef_initialize(args: *const MainArgs, settings: *mut Settings, app: *mut App, win_sandbox_info: *mut c_int ) -> c_int;
// 	fn cef_browser_host_create_browser(window_info: *const WindowInfo, client: *mut Client, url: *const CefString, browser_settings: *const BrowserSettings, request_context: *mut c_void ) -> c_int;
// 	fn cef_run_message_loop();
// 	fn cef_shutdown();
// }

pub fn execute_process(
  args: &MainArgs,
  app: &mut cef_app::CefApp,
  win_sandbox_info: *mut ::std::os::raw::c_void
) -> c_int {
  unsafe {
    cef_sys::cef_execute_process(
      args.into(),
      app.into(),
      win_sandbox_info
    )
  }
}

pub fn initialize(
  args: &MainArgs,
  settings: &cef_settings::CefAppSettings,
  app: &mut cef_app::CefApp
) -> c_int {
	unsafe {
    cef_sys::cef_initialize(
      args.into(),
      settings.into(),
      app.into(),
      std::ptr::null_mut()
    )
  }
}

pub fn browser_host_create_browser(
  window_info: *const CefWindowInfo,
  client: *mut cef_browser::CefClient,
  url: *const cef_string::CefString,
  browser_settings: &cef_settings::CefBrowserSettings,
  extra_info: *mut c_void,
  request_context: *mut c_void
) -> c_int {
	unsafe {
    return cef_browser_host_create_browser(
      window_info,
      client,
      url,
      browser_settings.into(),
      extra_info,
      request_context
    );
  }
}

pub fn browser_host_create_browser_sync(
  window_info: &CefWindowInfo,
  client: &mut cef_browser::CefClient,
  url: &cef_string::CefString,
  browser_settings: &cef_settings::CefBrowserSettings,
) -> *mut cef_browser::CefBrowser {
	unsafe {
    return cef_sys::cef_browser_host_create_browser_sync(
      window_info.into(),
      client.into(),
      url.into(),
      browser_settings.into(),
      std::ptr::null_mut(),
      std::ptr::null_mut()
    ) as *mut cef_browser::CefBrowser;
  }
}
pub fn run_message_loop() {
	unsafe { cef_run_message_loop(); }
}
pub fn do_message_loop_work() {
  unsafe { cef_do_message_loop_work(); }
}
pub fn shutdown(){
	unsafe { cef_shutdown() };
}

pub fn copy_cef_binaries_to_target() {
  cef_sys::copy_cef_binaries_to_target();
}
