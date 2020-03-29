#[cfg(target_os = "windows")]
pub mod windows {
  use std;
  use libc::{wchar_t, c_void};
  use std::os::raw::c_int;

  use cef_base;
  use cef_string;

  #[link(name = "kernel32")]
  extern "stdcall" {
    fn GetModuleHandleW(moduleName: *const wchar_t) -> *mut c_void;
  }

  #[repr(C)]
  pub struct MainArgs {
      instance: *mut c_void
  }

  impl From<&MainArgs> for *const cef_sys::_cef_main_args_t {
    fn from(rs: &MainArgs) -> Self {
      return cef_base::return_non_ref_counted_as_cef_sys(rs);
    }
  }

  impl MainArgs {
    pub fn get() -> MainArgs {
      let hinst = unsafe { GetModuleHandleW(std::ptr::null_mut()) };
      return MainArgs { instance: hinst }
    }
  }

  #[repr(C)]
  pub struct CefWindowInfo {
    sys: cef_sys::_cef_window_info_t,
  }

  impl From<&CefWindowInfo> for *const cef_sys::_cef_window_info_t {
    fn from(rs: &CefWindowInfo) -> Self {
      return cef_base::return_non_ref_counted_as_cef_sys(rs);
    }
  }

  #[allow(overflowing_literals)]
  impl CefWindowInfo {
    const CW_USEDEFAULT: c_int = 0x80000000;
    const WS_CLIPCHILDREN: c_int = 0x02000000;
    const WS_CLIPSIBLINGS: c_int = 0x04000000;
    const WS_VISIBLE: c_int = 0x10000000;
    const WS_TILEDWINDOW: c_int =  0x00C00000 | 0x00080000 | 0x00040000 | 0x00010000 | 0x00020000;
    const WS_CHILD: c_int = 0x40000000;
    const WS_TABSTOP: c_int =0x00010000;

    pub fn from(width: u32, height: u32) -> Self {
      let separate_window_window_style
        = CefWindowInfo::WS_CLIPCHILDREN
        | CefWindowInfo::WS_CLIPSIBLINGS
        | CefWindowInfo::WS_VISIBLE
        | CefWindowInfo::WS_TILEDWINDOW;

      let mut window_info = CefWindowInfo {
        sys: cef_sys::_cef_window_info_t::default()
      };

      window_info.sys.style = separate_window_window_style as cef_sys::DWORD;
      window_info.sys.window_name = cef_string::CefString::from("Rust CEF").into();
      window_info.sys.x = CefWindowInfo::CW_USEDEFAULT;
      window_info.sys.y = CefWindowInfo::CW_USEDEFAULT;
      window_info.sys.width = width as c_int;
      window_info.sys.height = height as c_int;

      window_info
      // return CefWindowInfo {
      //   sys: cef_sys::_cef_window_info_t {
      //     ex_style: 0u32,
      //     window_name: cef_string::CefString::from("Rust CEF").into(),
      //     style: separate_window_window_style,
      //     x: CefWindowInfo::CW_USEDEFAULT,
      //     y: CefWindowInfo::CW_USEDEFAULT,
      //     width: CefWindowInfo::CW_USEDEFAULT,
      //     height: CefWindowInfo::CW_USEDEFAULT,
      //     parent_window: std::ptr::null(),
      //     menu: std::ptr::null(),
      //     windowless_rendering_enabled: 0i32,
      //     shared_texture_enabled: 0,
      //     external_begin_frame_enabled: 0,
      //     window: std::ptr::null()
      //   }
      }

    #[allow(non_snake_case)]
    pub fn as_child_window(
      x: i32, y: i32, width: u32, height: u32, parent_window: *mut libc::c_void
    ) -> Self {
      let as_child_window_style
        = CefWindowInfo::WS_CHILD
        | CefWindowInfo::WS_CLIPCHILDREN
        | CefWindowInfo::WS_CLIPSIBLINGS
        | CefWindowInfo::WS_TABSTOP
        | CefWindowInfo::WS_VISIBLE;

      let mut window_info = CefWindowInfo::from(width, height);

      window_info.sys.style = as_child_window_style as cef_sys::DWORD;
      window_info.sys.x = x;
      window_info.sys.y = y;
      window_info.sys.parent_window = unsafe { std::mem::transmute(parent_window) };

      window_info
      // return CefWindowInfo {
      //   sys: cef_sys::_cef_window_info_t {
      //     style: as_child_window_style,
      //     .. cef_sys::_cef_window_info_t::default()
      //   },
      //   // .. CefWindowInfo::default()
      // }
    }
  }
}
