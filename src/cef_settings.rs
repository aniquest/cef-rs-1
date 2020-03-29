use std::env;
use cef_base;

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
pub struct CefAppSettings {
  sys: cef_sys::_cef_settings_t,
}

impl From<&CefAppSettings> for *const cef_sys::_cef_settings_t {
  fn from(rs: &CefAppSettings) -> Self {
    cef_base::return_non_ref_counted_as_cef_sys(rs)
  }
}

impl CefAppSettings {
  pub fn default() -> CefAppSettings {
    return CefAppSettings {
      sys: cef_sys::_cef_settings_t {
        no_sandbox: 1,
        multi_threaded_message_loop: 0,
        log_severity: 0, // info
        background_color: 0u32,
        ..cef_sys::_cef_settings_t::default()
      }
    }
  }
}

#[repr(C)]
pub struct CefBrowserSettings {
  sys: cef_sys::_cef_browser_settings_t,
}

impl From<&CefBrowserSettings> for *const cef_sys::_cef_browser_settings_t {
  fn from(rs: &CefBrowserSettings) -> Self {
    cef_base::return_non_ref_counted_as_cef_sys(rs)
  }
}

impl CefBrowserSettings {
  pub fn default() -> CefBrowserSettings {
    return CefBrowserSettings {
      sys: cef_sys::_cef_browser_settings_t::default()
    }
  }
}
