extern crate cef_sys;
extern crate libc;

extern crate cef_rs;

use cef_rs::cef_string::CefString;
use cef_rs::platform::windows::{MainArgs, WindowInfo};
use cef_rs::browser::{Client};

fn main() {
  println!("Cef execute process");

  let args = MainArgs::get();
	let mut app = cef_rs::app::App::new();

  let exec_ret = cef_rs::execute_process(&args, &mut app, std::ptr::null_mut());
  println!("execute_process returned {} ", exec_ret);
  if exec_ret >= 0 {
    return;
  }

  let mut settings = cef_rs::settings::AppSettings {
      log_file: CefString::from("cef_log.log"),
      .. cef_rs::settings::AppSettings::default()
  };

  // let mut settings = cef_rs::settings::AppSettings::default();
  println!("Cef initialize");
  let init_ret = cef_rs::initialize(&args, &mut settings, &mut app);
  if init_ret != 1 {
    println!("Init failed with {}", init_ret);
    return;
  }

  let mut client = Client::default();
  let url = CefString::from("www.google.com");
  let browser_settings = cef_rs::settings::BrowserSettings::default();

  // Set the window title and dimensions
  let window_info = WindowInfo {
      width: 1366,
      height: 768,
      .. WindowInfo::default()
  };

  println!("Cef browser host create browser");
  cef_rs::browser_host_create_browser(
    &window_info,
    &mut client,
    &url,
    &browser_settings,
    std::ptr::null_mut(),
    std::ptr::null_mut(),
  );

  println!("run cef's message loop");
  cef_rs::run_message_loop();

  println!("Cef shutdown");
  cef_rs::shutdown();
}
