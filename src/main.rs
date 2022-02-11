mod app;
mod document_list;
mod file_operations;
mod scribedown_window;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::Application;

fn main() {
    let app = gtk::Application::new(Some("org.alexisdumas.ScribeDown"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let win = scribedown_window::Window::new(app);

    // Create a thread-safe and memory-safe way to access UI and application state centrally
    let scribedown = Rc::new(RefCell::new(app::App {
        window: win,
        state: app::State {
            project: None,
            open_files: vec![],
        },
        document_list_model: None,
    }));

    // Wire up all the callbacks using that state
    app::App::connect_all(scribedown.clone());

    // Show the window!
    scribedown.borrow().window.show_all();
}
