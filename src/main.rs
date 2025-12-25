#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gtk::glib;
use gtk::Application;
use gtk::prelude::ApplicationExt;
use gtk::prelude::ApplicationExtManual;

const APP_ID: &str = "net.fwing.img_tournament";

// main ////////////////////////////////////////////////////
fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(|_| img_tournament::load_css());
    app.connect_activate(img_tournament::build_ui);
    let ret = app.run();
    ret
}
