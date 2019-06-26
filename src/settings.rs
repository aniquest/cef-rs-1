use libc::{c_int, size_t};
use cef_string::CefString;
use std::mem;
use std::env;

pub fn env_var<K: AsRef<std::ffi::OsStr>>(key: K) -> String {
    env::var(&key).expect(&format!("Unable to find env var {:?}", key.as_ref()))
}

// To enable the sandbox on Windows the following requirements must be met:
// 1. Use the same executable for the browser process and all sub-processes.
// 2. Link the executable with the cef_sandbox static library.
// 3. Call the cef_sandbox_info_create() function from within the executable
//    (not from a separate DLL) and pass the resulting pointer into both the
//    CefExecutProcess() and CefInitialize() functions via the
//    |windows_sandbox_info| parameter.

type CefColor = u32;
type CefLogSeverity = i32;

#[repr(C)]
pub struct AppSettings {
  pub size: usize,
  pub no_sandbox: ::std::os::raw::c_int,
  pub browser_subprocess_path: CefString,
  pub framework_dir_path: CefString,
  pub multi_threaded_message_loop: ::std::os::raw::c_int,
  pub external_message_pump: ::std::os::raw::c_int,
  pub windowless_rendering_enabled: ::std::os::raw::c_int,
  pub command_line_args_disabled: ::std::os::raw::c_int,
  pub cache_path: CefString,
  pub root_cache_path: CefString,
  pub user_data_path: CefString,
  pub persist_session_cookies: ::std::os::raw::c_int,
  pub persist_user_preferences: ::std::os::raw::c_int,
  pub user_agent: CefString,
  pub product_version: CefString,
  pub locale: CefString,
  pub log_file: CefString,
  pub log_severity: CefLogSeverity,
  pub javascript_flags: CefString,
  pub resources_dir_path: CefString,
  pub locales_dir_path: CefString,
  pub pack_loading_disabled: ::std::os::raw::c_int,
  pub remote_debugging_port: ::std::os::raw::c_int,
  pub uncaught_exception_stack_size: ::std::os::raw::c_int,
  pub ignore_certificate_errors: ::std::os::raw::c_int,
  pub enable_net_security_expiration: ::std::os::raw::c_int,
  pub background_color: CefColor,
  pub accept_language_list: CefString,
  pub application_client_id_for_file_scanning: CefString,
}

impl AppSettings {
  pub fn default() -> AppSettings {
    return AppSettings {
        size: mem::size_of::<AppSettings>() as size_t,
        no_sandbox: 1,
        browser_subprocess_path: CefString::empty(),
        framework_dir_path: CefString::empty(),
        multi_threaded_message_loop: 0,
        external_message_pump: 0,
        windowless_rendering_enabled: 0,
        command_line_args_disabled: 0,
        cache_path: CefString::empty(),
        root_cache_path: CefString::empty(),
        user_data_path: CefString::empty(),
        persist_session_cookies: 0,
        persist_user_preferences: 0,
        user_agent: CefString::empty(),
        product_version: CefString::empty(),
        locale: CefString::empty(),
        log_file: CefString::empty(),
        log_severity: 0, // info
        javascript_flags: CefString::empty(),
        resources_dir_path: CefString::empty(),
        locales_dir_path: CefString::empty(),
        pack_loading_disabled: 0,
        remote_debugging_port: 0,
        uncaught_exception_stack_size: 0,
        ignore_certificate_errors: 0,
        enable_net_security_expiration: 0,
        background_color: 0u32,
        accept_language_list: CefString::empty(),
        application_client_id_for_file_scanning: CefString::empty()
      }
  }
}

#[repr(C)]
pub enum CefState {
Default = 0,
Enabled,
Disabled
}

#[repr(C)]
pub struct BrowserSettings {
  pub size: usize,
  pub windowless_frame_rate: c_int,
  pub standard_font_family: CefString,
  pub fixed_font_family: CefString,
  pub serif_font_family: CefString,
  pub sans_serif_font_family: CefString,
  pub cursive_font_family: CefString,
  pub fantasy_font_family: CefString,
  pub default_font_size: c_int,
  pub default_fixed_font_size: c_int,
  pub minimum_font_size: c_int,
  pub minimum_logical_font_size: c_int,
  pub default_encoding: CefString,
  pub remote_fonts: CefState,
  pub javascript: CefState,
  pub javascript_close_windows: CefState,
  pub javascript_access_clipboard: CefState,
  pub javascript_dom_paste: CefState,
  pub plugins: CefState,
  pub universal_access_from_file_urls: CefState,
  pub file_access_from_file_urls: CefState,
  pub web_security: CefState,
  pub image_loading: CefState,
  pub image_shrink_standalone_to_fit: CefState,
  pub text_area_resize: CefState,
  pub tab_to_links: CefState,
  pub local_storage: CefState,
  pub databases: CefState,
  pub application_cache: CefState,
  pub webgl: CefState,
  pub background_color: CefColor,
  pub accept_language_list: CefString,
}

impl BrowserSettings {
  pub fn default() -> BrowserSettings {
    return BrowserSettings {
      size: mem::size_of::<Self>() as size_t,
      windowless_frame_rate: 0,
      standard_font_family: CefString::empty(),
      fixed_font_family: CefString::empty(),
      serif_font_family: CefString::empty(),
      sans_serif_font_family: CefString::empty(),
      cursive_font_family: CefString::empty(),
      fantasy_font_family: CefString::empty(),
      default_font_size: 0,
      default_fixed_font_size: 0,
      minimum_font_size: 0,
      minimum_logical_font_size: 0,
      default_encoding: CefString::empty(),
      remote_fonts: CefState::Default,
      javascript: CefState::Default,
      javascript_close_windows: CefState::Default,
      javascript_access_clipboard: CefState::Default,
      javascript_dom_paste: CefState::Default,
      plugins: CefState::Default,
      universal_access_from_file_urls: CefState::Default,
      file_access_from_file_urls: CefState::Default,
      web_security: CefState::Default,
      image_loading: CefState::Default,
      image_shrink_standalone_to_fit: CefState::Default,
      text_area_resize: CefState::Default,
      tab_to_links: CefState::Default,
      local_storage: CefState::Default,
      databases: CefState::Default,
      application_cache: CefState::Default,
      webgl: CefState::Default,
      background_color: 0,
      accept_language_list: CefString::empty(),
    }
  }
}
