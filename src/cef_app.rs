use cef_base;
use cef_browser;
use cef_types::*;
use cef_v8::CefV8Context;
use process_message::{CefProcessId, CefProcessMessage};
use std::os::raw::c_int;

type ContextCreatedHandler =
    Box<Fn(&mut cef_browser::CefBrowser, &mut cef_browser::CefFrame, &mut CefV8Context)>;
type ProcessMessageReceivedHandler = Box<
    Fn(
        &mut cef_browser::CefBrowser,
        &mut cef_browser::CefFrame,
        CefProcessId,
        &mut CefProcessMessage,
    ) -> c_int,
>;

#[repr(C)]
struct CefRenderProcessHandlerSys {
    base: cef_sys::cef_base_ref_counted_t,
    pub on_render_thread_created:
        Option<extern "C" fn(self_: *mut CefRenderProcessHandler, extra_info: *mut CefListValue)>,
    pub on_web_kit_initialized: Option<extern "C" fn(self_: *mut CefRenderProcessHandler)>,
    pub on_browser_created: Option<
        extern "C" fn(
            self_: *mut CefRenderProcessHandler,
            browser: *mut cef_browser::CefBrowser,
            extra_info: *mut CefDictionaryValue,
        ),
    >,
    pub on_browser_destroyed: Option<
        extern "C" fn(self_: *mut CefRenderProcessHandler, browser: *mut cef_browser::CefBrowser),
    >,
    pub get_load_handler:
        Option<extern "C" fn(self_: *mut CefRenderProcessHandler) -> *mut CefLoadHandler>,
    pub on_context_created: Option<
        extern "C" fn(
            self_: *mut CefRenderProcessHandler,
            browser: *mut cef_browser::CefBrowser,
            frame: *mut cef_browser::CefFrame,
            context: *mut cef_sys::_cef_v8context_t,
        ),
    >,
    pub on_context_released: Option<
        extern "C" fn(
            self_: *mut CefRenderProcessHandler,
            browser: *mut cef_browser::CefBrowser,
            frame: *mut cef_browser::CefFrame,
            context: *mut cef_sys::_cef_v8context_t,
        ),
    >,
    pub on_uncaught_exception: Option<
        extern "C" fn(
            self_: *mut CefRenderProcessHandler,
            browser: *mut cef_browser::CefBrowser,
            frame: *mut cef_browser::CefFrame,
            context: *mut cef_sys::_cef_v8context_t,
            exception: *mut CefV8Exception,
            stackTrace: *mut CefV8StackTrace,
        ),
    >,
    pub on_focused_node_changed: Option<
        extern "C" fn(
            self_: *mut CefRenderProcessHandler,
            browser: *mut cef_browser::CefBrowser,
            frame: *mut cef_browser::CefFrame,
            node: *mut CefDomNode,
        ),
    >,
    pub on_process_message_received: Option<
        extern "C" fn(
            self_: *mut CefRenderProcessHandler,
            browser: *mut cef_browser::CefBrowser,
            frame: *mut cef_browser::CefFrame,
            source_process: CefProcessId,
            message: *mut CefProcessMessage,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct CefRenderProcessHandler {
    // cef_sys::cef_render_process_handler_t members.
    base: cef_base::CefRefCounted<CefRenderProcessHandlerSys>,

    // Rust callbacks.
    context_created_handler: Option<ContextCreatedHandler>,
    process_message_received_handler: Option<ProcessMessageReceivedHandler>,
}

impl From<&mut CefRenderProcessHandler> for *mut cef_sys::_cef_render_process_handler_t {
    fn from(rs: &mut CefRenderProcessHandler) -> Self {
        cef_base::add_ref_and_return_as_cef_sys(rs)
    }
}

impl CefRenderProcessHandler {
    pub fn new(
        context_created_handler: Option<ContextCreatedHandler>,
        process_message_received_handler: Option<ProcessMessageReceivedHandler>,
    ) -> CefRenderProcessHandler {
        let sys = CefRenderProcessHandlerSys {
            base: cef_base::make_empty_cef_base(),
            on_render_thread_created: None,
            on_web_kit_initialized: None,
            on_browser_created: None,
            on_browser_destroyed: None,
            get_load_handler: None,
            on_context_created: Some(CefRenderProcessHandler::on_context_created),
            on_context_released: None,
            on_uncaught_exception: None,
            on_focused_node_changed: None,
            on_process_message_received: Some(CefRenderProcessHandler::on_process_message_received),
        };

        let vtable = cef_base::CefRefCounted::<CefRenderProcessHandlerSys>::new(sys);

        return CefRenderProcessHandler {
            base: vtable,

            // Our state
            context_created_handler,
            process_message_received_handler,
        };
    }

    extern "C" fn on_context_created(
        self_: *mut CefRenderProcessHandler,
        browser: *mut cef_browser::CefBrowser,
        frame: *mut cef_browser::CefFrame,
        context_sys: *mut cef_sys::_cef_v8context_t,
    ) {
        unsafe {
            match (*self_).context_created_handler {
                Some(ref mut handler) => {
                    let mut context = CefV8Context::from(context_sys);

                    handler(&mut *browser, &mut *frame, &mut context)
                }
                None => {}
            }
        }
    }

    extern "C" fn on_process_message_received(
        self_: *mut CefRenderProcessHandler,
        browser: *mut cef_browser::CefBrowser,
        frame: *mut cef_browser::CefFrame,
        source_process: cef_sys::cef_process_id_t,
        message: *mut CefProcessMessage,
    ) -> c_int {
        unsafe {
            match (*self_).process_message_received_handler {
                Some(ref mut handler) => {
                    handler(&mut *browser, &mut *frame, source_process, &mut *message)
                }
                None => 0,
            }
        }
    }
}

type ContextInitializedHandler = Box<Fn()>;

#[repr(C)]
pub struct CefBrowserProcessHandlerSys {
    base: cef_sys::cef_base_ref_counted_t,
    pub on_context_initialized: Option<extern "C" fn(self_: *mut CefBrowserProcessHandler)>,
    pub on_before_child_process_launch: Option<
        extern "C" fn(
            self_: *mut CefBrowserProcessHandler,
            command_line: *mut cef_sys::_cef_command_line_t,
        ),
    >,
    pub on_render_process_thread_created: Option<
        extern "C" fn(
            self_: *mut CefBrowserProcessHandler,
            extra_info: *mut cef_sys::_cef_list_value_t,
        ),
    >,
    pub get_print_handler: Option<
        extern "C" fn(self_: *mut CefBrowserProcessHandler) -> *mut cef_sys::_cef_print_handler_t,
    >,
    pub on_schedule_message_pump_work:
        Option<extern "C" fn(self_: *mut CefBrowserProcessHandler, delay_ms: i64)>,
}

#[repr(C)]
pub struct CefBrowserProcessHandler {
    // cef_app_t members
    pub base: cef_base::CefRefCounted<CefBrowserProcessHandlerSys>,

    // Rust callbacks.
    context_initialized_handler: Option<ContextInitializedHandler>,
}

impl From<&mut CefBrowserProcessHandler> for *mut cef_sys::_cef_browser_process_handler_t {
    fn from(rs: &mut CefBrowserProcessHandler) -> Self {
        cef_base::add_ref_and_return_as_cef_sys(rs)
    }
}

impl CefBrowserProcessHandler {
    pub fn new(
        context_initialized_handler: Option<ContextInitializedHandler>,
    ) -> CefBrowserProcessHandler {
        let sys = CefBrowserProcessHandlerSys {
            base: cef_base::make_empty_cef_base(),
            on_context_initialized: Some(CefBrowserProcessHandler::on_context_initialized),
            on_before_child_process_launch: None,
            on_render_process_thread_created: None,
            get_print_handler: None,
            on_schedule_message_pump_work: None,
        };

        let vtable = cef_base::CefRefCounted::<CefBrowserProcessHandlerSys>::new(sys);

        return CefBrowserProcessHandler {
            base: vtable,

            // Our state
            context_initialized_handler,
        };
    }

    extern "C" fn on_context_initialized(self_: *mut CefBrowserProcessHandler) {
        unsafe {
            match (*self_).context_initialized_handler {
                Some(ref mut handler) => handler(),
                None => {}
            }
        }
    }
}

#[repr(C)]
struct AppSys {
    base: cef_sys::cef_base_ref_counted_t,
    on_before_command_line_processing: Option<extern "C" fn(this: *mut CefApp)>,
    on_register_custom_schemes: Option<extern "C" fn(this: *mut CefApp)>,
    get_resource_bundle_handler:
        Option<extern "C" fn(this: *mut CefApp) -> *mut cef_sys::_cef_resource_bundle_handler_t>,
    get_browser_process_handler:
        Option<extern "C" fn(this: *mut CefApp) -> *mut cef_sys::_cef_browser_process_handler_t>,
    get_render_process_handler:
        Option<extern "C" fn(this: *mut CefApp) -> *mut cef_sys::_cef_render_process_handler_t>,
}

#[repr(C)]
pub struct CefApp {
    // cef_app_t members
    vtable: cef_base::CefRefCounted<AppSys>,

    // Rust callbacks.
    browser_process_handler: Option<CefBrowserProcessHandler>,
    render_process_handler: Option<CefRenderProcessHandler>,
}

// https://bitbucket.org/chromiumembedded/cef/wiki/UsingTheCAPI
// Before passing a Cef structure into a function call on the cef dll-side, we must first bump
// the reference count.
// Implement this trait for any reference-counted Cef object and call .into() to get a
// pointer to the C sys object.
impl From<&mut CefApp> for *mut cef_sys::_cef_app_t {
    fn from(rs: &mut CefApp) -> Self {
        cef_base::add_ref_and_return_as_cef_sys(rs)
    }
}

impl CefApp {
    pub fn new(
        render_process_handler: Option<CefRenderProcessHandler>,
        browser_process_handler: Option<CefBrowserProcessHandler>,
    ) -> CefApp {
        let sys = AppSys {
            base: cef_base::make_empty_cef_base(),
            on_before_command_line_processing: None,
            on_register_custom_schemes: None,
            get_resource_bundle_handler: None,
            get_browser_process_handler: Some(CefApp::get_browser_process_handler),
            get_render_process_handler: Some(CefApp::get_render_process_handler),
        };

        let vtable = cef_base::CefRefCounted::<AppSys>::new(sys);

        return CefApp {
            vtable,

            // Our state
            render_process_handler,
            browser_process_handler,
        };
    }

    extern "C" fn get_render_process_handler(
        this: *mut CefApp,
    ) -> *mut cef_sys::_cef_render_process_handler_t {
        unsafe {
            match (*this).render_process_handler {
                Some(ref mut handler) => handler.into(),
                None => std::ptr::null_mut(),
            }
        }
    }

    extern "C" fn get_browser_process_handler(
        this: *mut CefApp,
    ) -> *mut cef_sys::_cef_browser_process_handler_t {
        unsafe {
            match (*this).browser_process_handler {
                Some(ref mut handler) => handler.into(),
                None => std::ptr::null_mut(),
            }
        }
    }
}
