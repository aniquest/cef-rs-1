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
    const CW_USEDEFAULT: u32 = 0x80000000u32;
    const WS_CLIPCHILDREN: u32 = 0x02000000u32;
    const WS_CLIPSIBLINGS: u32 = 0x04000000u32;
    const WS_VISIBLE: u32 = 0x10000000u32;
    const WS_TILEDWINDOW: u32 =  0x00C00000u32 | 0x00080000u32 | 0x00040000u32 | 0x00010000u32 | 0x00020000u32;
    const WS_CHILD: u32 = 0x40000000u32;
    const WS_TABSTOP: u32 =0x00010000u32;

    pub fn default() -> Self {
      let separate_window_window_style
        = WindowInfo::WS_CLIPCHILDREN
        | WindowInfo::WS_CLIPSIBLINGS
        | WindowInfo::WS_VISIBLE
        | WindowInfo::WS_TILEDWINDOW;

      return WindowInfo {
        ex_style: 0u32,
        window_name: CefString::from("Rust CEF"),
        style: separate_window_window_style,
        x: WindowInfo::CW_USEDEFAULT,
        y: WindowInfo::CW_USEDEFAULT,
        width: WindowInfo::CW_USEDEFAULT,
        height: WindowInfo::CW_USEDEFAULT,
        parent_window: std::ptr::null(),
        menu: std::ptr::null(),
        windowless_rendering_enabled: 0i32,
        shared_texture_enabled: 0,
        external_begin_frame_enabled: 0,
        window: std::ptr::null()
      }
    }

    #[allow(non_snake_case)]
    pub fn as_child_window() -> Self {
      let as_child_window_style
        = WindowInfo::WS_CHILD
        | WindowInfo::WS_CLIPCHILDREN
        | WindowInfo::WS_CLIPSIBLINGS
        | WindowInfo::WS_TABSTOP
        | WindowInfo::WS_VISIBLE;

      return WindowInfo {
        style: as_child_window_style,
        .. WindowInfo::default()
      }
    }
  }
}
