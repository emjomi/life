mod game;
mod ui;

use adw::{prelude::*, glib, Application};
use ui::build_ui;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.github.emjomi.Life")
        .build();
    
    app.connect_activate(build_ui);
    app.run()
}
