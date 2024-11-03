//Add gtk modules and Cell for memory management.
mod gui;
mod gui_calc;
use gtk::{prelude::*, glib, Application};


// choose a domain for your app.
const APP_ID: &str = "org.cnegasuy.NewGTKApp";

fn main() -> glib::ExitCode {
    // Create the app and acttivate it with "build_ui" func.
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(gui_calc::build_ui);
    app.run()
}
