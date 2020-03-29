extern crate cef_sys;
extern crate libc;

extern crate cef_rs;

use cef_rs::cef_app;
use cef_rs::cef_browser;
use cef_rs::cef_settings;
use cef_rs::cef_string::CefString;
use cef_rs::cef_v8::{
    CefExecuteHandlerResult, CefV8Context, CefV8Handler, CefV8Value, V8_PROPERTY_ATTRIBUTE_NONE,
};
use cef_rs::platform::windows;
use cef_rs::platform::windows::{CefWindowInfo, MainArgs};

fn main() {
    println!("Cef execute process");

    let args = MainArgs::get();

    let render_process_handler = cef_app::CefRenderProcessHandler::new(
        Some(Box::new(
            move |_browser, _frame, context: &mut CefV8Context| {
                // Retrieve the context's window object.
                let object = context.get_global();

                let handler = CefV8Handler::new(Some(Box::new(move |name, object, arguments| {
                    println!("rust-backed js function called.");
                    // println!("rust-backed js function called: {}", name);
                    return CefExecuteHandlerResult::Error(String::from("JS CEF Error!"));
                })));
                // CefRefPtr<CefV8Handler> handler = new MyV8Handler(browser, data_shared_with_render_process_);

                let rust_func = CefV8Value::create_function(String::from("rustFunc"), handler);
                // object.set_value_bykey(
                //   String::from("rustFunc"), rustFunc, V8_PROPERTY_ATTRIBUTE_NONE
                // );

                // _context.
                println!("render process handler on context created")
            },
        )),
        Some(Box::new(move |browser, frame, source_process, message| {
            unsafe {
                let name = &*(message.get_name)(message);
            }
            return 0;
        })),
    );

    let browser_process_handler =
        cef_app::CefBrowserProcessHandler::new(Some(Box::new(move || {
            println!("on context initialized");
        })));

    let mut app = cef_app::CefApp::new(Some(render_process_handler), Some(browser_process_handler));

    let exec_ret = cef_rs::execute_process(&args, &mut app, std::ptr::null_mut());
    println!("execute_process returned {} ", exec_ret);
    if exec_ret >= 0 {
        return;
    }

    let mut settings = cef_settings::CefAppSettings::default();

    println!("Cef initialize");
    let init_ret = cef_rs::initialize(&args, &mut settings, &mut app);
    if init_ret != 1 {
        println!("Init failed with {}", init_ret);
        return;
    }

    let mut client = cef_browser::CefClient::default();
    let url = CefString::from("www.google.com");
    let browser_settings = cef_settings::CefBrowserSettings::default();

    // Set the window title and dimensions
    let window_info = CefWindowInfo::from(1366, 768);

    println!("Cef browser host create browser");
    let browser = cef_rs::browser_host_create_browser_sync(
        &window_info,
        &mut client,
        &url,
        &browser_settings,
    );

    // let mut message = cef_rs::process_message::CefProcessMessage::new("CEF Inter-process message!");
    unsafe {
        let message = cef_sys::cef_process_message_create(
            cef_rs::cef_string::CefString::from("CEF Inter-process message!").as_sys(),
        );
        let frame = ((*browser).get_main_frame)(browser);
        ((*frame).send_process_message)(
            frame,
            cef_rs::process_message::CEF_PROCESS_ID_PID_RENDERER,
            message,
        );
    }

    println!("run cef's message loop");
    cef_rs::run_message_loop();

    println!("Cef shutdown");
    cef_rs::shutdown();
}
