use base::CefBase;

#[repr(C)]
pub struct App {
    base: CefBase,
    on_before_command_line_processing: extern fn(this: *mut App),
    on_register_custom_schemes: extern fn(this: *mut App),
    get_resource_bundle_handler: extern fn(this: *mut App)-> libc::c_int,
    get_browser_process_handler: extern fn(this: *mut App)-> libc::c_int,
    get_render_process_handler: extern fn(this: *mut App) -> libc::c_int
}

impl App {
    pub fn new() -> App {
        return App {
            base: CefBase::get::<App>(),
            on_before_command_line_processing: nop_app,
            on_register_custom_schemes: nop_app,
            get_resource_bundle_handler: nop_app_ptr,
            get_browser_process_handler: nop_app_ptr,
            get_render_process_handler: nop_app_ptr
        }
    }
}

#[allow(unused_variables)]
extern fn nop_app(ptr: *mut App) {
    return
}

#[allow(unused_variables)]
extern fn nop_app_ptr(ptr: *mut App) -> libc::c_int{
    return 0;
}
