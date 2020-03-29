use std::os::raw::c_int;
use cef_base;
use cef_string::CefString;
use cef_types::{CefTask};
use process_message::CefThreadId;
use cef_sys::*;
use std::ffi::CString;

pub type V8PropertyAttribute = i32;
pub const V8_PROPERTY_ATTRIBUTE_NONE: i32 = 0;
pub const V8_PROPERTY_ATTRIBUTE_READONLY: i32 = 1;
pub const V8_PROPERTY_ATTRIBUTE_DONTENUM: i32 = 2;
pub const V8_PROPERTY_ATTRIBUTE_DONTDELETE: i32 = 4;

// #[cfg(target_os="windows")]
// #[link(name = "libcef", kind = "dylib")]
// extern "C" {
//   fn cef_v8value_create_function(
//         name: *const CefString,
//         handler: *mut _cef_v8handler_t,
//     ) -> *mut _cef_v8value_t;
// }

// pub fn create_function(name: CefString, handler: &mut CefV8Handler) -> CefV8Value {
// 	unsafe {
//     return CefV8Value::from(cef_v8value_create_function(&name, handler.sys));
//   }
// }

pub enum CefExecuteHandlerResult {
  Retval(CefV8Value), // Return value.
  Error(String) // Message of exception that will be thrown if there was an error.
}

///
// Handle execution of the function identified by |name|. |object| is the
// receiver ('this' object) of the function. |arguments| is the list of
// arguments passed to the function. If execution succeeds set |retval| to the
// function return value. If execution fails set |exception| to the exception
// that will be thrown. Return true if execution was handled.
///
pub type CefExecuteHandler = Box<Fn(
  /*name */CefString,
  /*object: */CefV8Value,
  /*arguments: */Vec<CefV8Value>,
) -> CefExecuteHandlerResult>;

#[repr(C)]
pub struct CefV8Handler {
  // cef_sys members
  pub base: cef_base::CefBase,
  pub execute: Option<unsafe extern "C" fn(
    self_: *mut CefV8Handler,
    name: *const cef_sys::cef_string_t,
    object: *mut cef_sys::_cef_v8value_t,
    argumentsCount: usize,
    arguments: *const *mut cef_sys::_cef_v8value_t,
    retval: *mut *mut cef_sys::_cef_v8value_t,
    exception: *mut cef_sys::cef_string_t,
  ) -> c_int>,

  // Rust callbacks.
  execute_handler: Option<CefExecuteHandler>,
}

impl CefV8Handler {
  pub fn new(
    execute_handler: Option<CefExecuteHandler>
  ) -> CefV8Handler {
    return CefV8Handler {
      base: cef_base::CefBase::get::<CefV8Handler>(),
      execute: Some(CefV8Handler::execute),

      // Our state
      execute_handler
    };
  }

  pub fn as_sys(&self) -> *mut cef_sys::_cef_v8handler_t {
    let sys: *mut cef_sys::_cef_v8handler_t = unsafe {
      std::mem::transmute_copy(&self)
    };

    return sys;
    // return cef_sys::_cef_v8handler_t {
    //   base: self.base,
    //   execute: self.execute
    // };
  }

  extern "C" fn execute(
    this: *mut CefV8Handler,
    name: *const cef_string_t,
    object: *mut _cef_v8value_t,
    arguments_count: usize,
    arguments_sys: *const *mut _cef_v8value_t,
    retval: *mut *mut _cef_v8value_t,
    exception: *mut cef_string_t,
  ) -> c_int {
println!("v8 handler execute() called");

    unsafe {
      match (*this).execute_handler {
        Some(ref mut handler) => {
          let mut arguments = Vec::new();
          for i in 0..arguments_count {
            arguments.push(CefV8Value::from_sys(*arguments_sys.offset(i as isize)));
          }

          let result = handler(
            CefString::from_sys(name),
            CefV8Value::from_sys(object),
            arguments,
            // return_value,
            // exception
          );

          match result {
            CefExecuteHandlerResult::Retval(return_value) => {
              *retval = return_value.sys;
            },
            CefExecuteHandlerResult::Error(error_message) => {
              *exception = *CefString::from_string(error_message).as_sys();
            }
          };

          return 0;
        },
        None                  => 0
      }
    }
  }
}

#[repr(C)]
pub struct CefV8Context {
  sys: *mut cef_sys::_cef_v8context_t,
}

impl CefV8Context {
  pub fn from(sys: *mut cef_sys::_cef_v8context_t) -> CefV8Context {
    CefV8Context {
      sys
    }
  }

  pub fn get_global(&self) -> CefV8Value {
    let value_sys = unsafe { ((*self.sys).get_global.unwrap())(self.sys) };

    return CefV8Value::from_sys(value_sys);
  }
}

#[repr(C)]
pub struct CefV8Value {
  sys: *mut _cef_v8value_t,
}

pub fn cef_str_opt<'a>(s: Option<&'a str>) -> cef_sys::cef_string_t {
  match s {
    None => unsafe { std::mem::zeroed() },
    Some(s) => unsafe {
      let mut new_s: cef_sys::cef_string_utf16_t = std::mem::zeroed();
      let cstr = CString::new(s).unwrap();
      // TODO: I am expected to call "dtor" on this during release
      let ret = cef_sys::cef_string_utf8_to_utf16(cstr.as_ptr(), s.len(), &mut new_s);
      assert_eq!(ret, 1);
      new_s
    }
  }
}

impl CefV8Value {
  pub fn new() -> CefV8Value {
    return CefV8Value {
      sys: std::ptr::null_mut()
    };
  }

  pub fn from_sys(sys: *mut _cef_v8value_t) -> CefV8Value {
    return CefV8Value {
      sys
    };
  }

  pub fn create_function(name: String, handler: CefV8Handler) -> CefV8Value {
    let str = CefString::from_string(name).as_sys();

    let ret_val = unsafe {
      cef_sys::cef_v8value_create_function(
        // &mut cef_string_t::default(),
        // &mut cef_str_opt(Some("rustFunc")),
        // CefString::empty().as_sys(),
        str,
        // &mut _cef_v8handler_t::default()
        // std::mem::transmute(&_cef_v8handler_t2::default())
        // std::mem::transmute(&handler)
        handler.as_sys()
        // hndlr.as_sys()
      )
    };

    // return CefV8Value::new();
    return CefV8Value::from_sys(ret_val);
  }

  pub fn set_value_bykey(
    &self, 
    key: String,
    value: CefV8Value,
    attribute: V8PropertyAttribute,
  ) -> c_int {
    let sys = self.sys;
    return unsafe {
      ((*sys).set_value_bykey.unwrap())(sys, CefString::from_string(key).as_sys(), value.sys, attribute)
    };
  }

}

// pub struct CefV8Value {
//   sys: cef_sys::_cef_v8value_t,
// }

// impl CefV8Value {
//   pub fn from(sys: cef_sys::_cef_v8value_t) -> CefV8Value {
//     CefV8Value {
//       sys
//     }
//   }
// }

#[repr(C)]
pub struct CefTaskRunner {
    pub base: cef_base::CefBase,
    pub is_same: Option<extern "C" fn(self_: *mut CefTaskRunner, that: *mut CefTaskRunner, ) -> c_int,>,
    pub belongs_to_current_thread: Option<extern "C" fn(self_: *mut CefTaskRunner) -> c_int,>,
    pub belongs_to_thread: Option<extern "C" fn(self_: *mut CefTaskRunner, threadId: CefThreadId,) -> c_int,>,
    pub post_task: Option<extern "C" fn(self_: *mut CefTaskRunner, task: *mut CefTask,) -> c_int,>,
    pub post_delayed_task: Option<extern "C" fn(self_: *mut CefTaskRunner, task: *mut CefTask, delay_ms: i64,) -> c_int,>,
}
