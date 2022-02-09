mod scribedown_window;
use gtk::prelude::*;
use gtk::Application;

fn main() {
    let application = gtk::Application::new(Some("org.alexisdumas.ScribeDown"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &Application) {
    let win = scribedown_window::Window::new(app);
    win.show();
}
