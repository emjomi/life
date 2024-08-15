mod game;
mod ui;

use gtk::{prelude::*, glib, Application};
use ui::build_ui;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.life.Life")
        .build();
    
    app.connect_activate(build_ui);
    app.run()
}
