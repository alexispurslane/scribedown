mod app;
mod document_list;
mod file_operations;
mod gtk_utils;
mod macro_utils;
mod markdown_editor;
mod scribedown_window;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::gdk::Screen;
use gtk::prelude::*;
use gtk::Application;
use gtk::CssProvider;
use gtk::StyleContext;
use gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

fn main() {
    let app = gtk::Application::new(Some("org.alexisdumas.ScribeDown"), Default::default());
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // Set up CSS providor, to load CSS styling for the text view
    let provider = CssProvider::new();
    provider
        .load_from_data(include_bytes!("style.css"))
        .expect("Couldn't load css");
    StyleContext::add_provider_for_screen(
        &Screen::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

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
