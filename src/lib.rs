#![crate_type = "lib"]

pub mod cef_string;
pub mod base;
pub mod app;
pub mod browser;
pub mod platform;
pub mod settings;

extern crate cef_sys;
extern crate libc;
use libc::{c_int, c_void};

use app::App;
use cef_string::CefString;
use settings::{AppSettings, BrowserSettings};
use browser::{Client};
// use platform::linux::{WindowInfo, MainArgs};
use platform::windows::{WindowInfo, MainArgs};

#[cfg(target_os="windows")]
#[link(name = "libcef", kind = "dylib")]
extern "C" {
// extern "stdcall" { //On windows32 stdcall. on 64 use C?
	fn cef_execute_process(args: *const MainArgs, app: *mut App, win_sandbox_info: *mut c_int) -> c_int;
	fn cef_initialize(args: *const MainArgs, settings: *mut AppSettings, app: *mut App, win_sandbox_info: *mut c_int ) -> c_int;
	fn cef_browser_host_create_browser(window_info: *const WindowInfo, client: *mut Client, url: *const CefString, browser_settings: *const BrowserSettings, extra_info: *mut c_void, request_context: *mut c_void) -> c_int;
	fn cef_browser_host_create_browser_sync(window_info: *const WindowInfo, client: *mut Client, url: *const CefString, browser_settings: *const BrowserSettings, extra_info: *mut c_void, request_context: *mut c_void) -> c_int;
	fn cef_run_message_loop();
  fn cef_do_message_loop_work();
	fn cef_shutdown();
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

pub fn execute_process(args: *const MainArgs, app: *mut App, win_sandbox_info: *mut c_int) -> c_int {
	unsafe { return cef_execute_process(args, app, win_sandbox_info); }
}
pub fn initialize(args: *const MainArgs, settings: *mut AppSettings, app: *mut App ) -> c_int {
	unsafe { return cef_initialize(args, settings, app, std::ptr::null_mut()) }
}
pub fn browser_host_create_browser(
  window_info: *const WindowInfo,
  client: *mut Client,
  url: *const CefString,
  browser_settings: *const BrowserSettings,
  extra_info: *mut c_void,
  request_context: *mut c_void
) -> c_int {
	unsafe { return cef_browser_host_create_browser(window_info, client, url, browser_settings, extra_info, request_context); }
}
pub fn browser_host_create_browser_sync(
  window_info: *const WindowInfo,
  client: *mut Client,
  url: *const CefString,
  browser_settings: *const BrowserSettings,
  extra_info: *mut c_void,
  request_context: *mut c_void
) -> c_int {
	unsafe { return cef_browser_host_create_browser_sync(window_info, client, url, browser_settings, extra_info, request_context); }
}
pub fn run_message_loop(){
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
