mod scribedown_window;
use std::sync::mpsc::{channel, Receiver, Sender};

use gtk::prelude::*;
use gtk::Application;

fn main() {
    let app = gtk::Application::new(Some("org.alexisdumas.ScribeDown"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    use scribedown_window::*;
    let win = Window::new(app);
    win.show();
}
