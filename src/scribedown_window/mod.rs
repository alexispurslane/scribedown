use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio, glib};

mod imp;

pub enum ScribeDownWindowEvent {
    OpenProject(String),
    OpenFile(String),
    OpenFileNewTab(String),
    CloseTab(u32),
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Window {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create ScribeDownWindow")
    }

    fn setup_callbacks(&self) {}
}

async fn folder_choose_dialog(state: Rc<RefCell<imp::ScribeDownWindowState>>) {
    let folder_dialog = gtk::FileChooserDialog::builder()
        .title("Open project folder...")
        .action(gtk::FileChooserAction::SelectFolder)
        .build();
    folder_dialog.add_button("Open Folder", gtk::ResponseType::Accept);
    folder_dialog.add_button("Cancel", gtk::ResponseType::Cancel);

    let folder_path = folder_dialog.run_future().await;
    match folder_path {
        gtk::ResponseType::Accept => {
            state.borrow_mut().project_path = Some(String::from(
                folder_dialog.filename().unwrap().to_string_lossy(),
            ));
        }
        gtk::ResponseType::Cancel => println!("Nevermind!"),
        _ => unreachable!(),
    }
    folder_dialog.close();
}
