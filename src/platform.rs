#[cfg(target_os = "windows")]
pub mod windows {
  use std;
  use libc::wchar_t;
  use libc::c_void;
  use libc::c_int;

  use cef_string::CefString;

  #[link(name = "kernel32")]
  extern "stdcall" {
    fn GetModuleHandleW(moduleName: *const wchar_t) -> *mut c_void;
  }

  #[repr(C)]
  pub struct MainArgs {
      instance: *mut c_void
  }

  impl MainArgs {
    pub fn get() -> MainArgs {
      let hinst = unsafe { GetModuleHandleW(std::ptr::null_mut()) };
      return MainArgs { instance: hinst }
    }
  }

  #[repr(C)]
  pub struct WindowInfo {
    pub ex_style: u32,
    pub window_name: CefString,
    pub style: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub parent_window: *const c_void,
    pub menu: *const c_void,
    pub windowless_rendering_enabled: c_int,
    pub shared_texture_enabled: c_int,
    pub external_begin_frame_enabled: c_int,
    pub window: *const c_void,
  }
  impl WindowInfo {
    #[allow(non_snake_case)]
    pub fn default() -> Self {
      let CW_USEDEFAULT = 0x80000000u32;
      let WS_CLIPCHILDREN = 0x02000000u32;
      let WS_CLIPSIBLINGS = 0x04000000u32;
      let WS_VISIBLE = 0x10000000u32;
      let WS_TILEDWINDOW =  0x00C00000u32 | 0x00080000u32 | 0x00040000u32 | 0x00010000u32 | 0x00020000u32;
      let window_style = WS_CLIPCHILDREN | WS_CLIPSIBLINGS | WS_VISIBLE | WS_TILEDWINDOW;
      return WindowInfo {
        ex_style: 0u32,
        window_name: CefString::from("Rust CEF"),
        style: window_style,
        x: CW_USEDEFAULT,
        y: CW_USEDEFAULT,
        width: CW_USEDEFAULT,
        height: CW_USEDEFAULT,
        parent_window: std::ptr::null(),
        menu: std::ptr::null(),
        windowless_rendering_enabled: 0i32,
        shared_texture_enabled: 0,
        external_begin_frame_enabled: 0,
        window: std::ptr::null()
      }
    }
  }
}
