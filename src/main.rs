mod app;
mod scribedown_window;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};

use gtk::prelude::*;
use gtk::Application;

fn main() {
    let app = gtk::Application::new(Some("org.alexisdumas.ScribeDown"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let win = scribedown_window::Window::new(app);
    let scribedown = Rc::new(RefCell::new(app::App {
        window: win,
        state: app::State {
            project: None,
            open_files: vec![]
        }
    }));
    app::App::connect_all(scribedown.clone());
    scribedown.borrow().window.show_all();
}
