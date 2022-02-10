use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Project {
    pub path: String,
    pub title: String,
    pub docs: HashMap<String, Document>,
}

pub struct Document {
    pub path: String,
    pub title: String,
    pub contents: Option<String>,
}

pub struct State {
    pub project: Option<Project>,
    pub open_files: Vec<String>,
}
pub struct App {
    pub window: crate::scribedown_window::Window,
    pub state: State,
}

impl App {
    pub fn connect_all(sd: Rc<RefCell<Self>>) {
        let outer_sd = sd.clone();
        let osd = outer_sd.borrow();
        let button = &osd.window.imp().open_button;
        button.connect_clicked(move |_| {
            gtk::glib::MainContext::default().spawn_local(Self::open_project_dialog(Rc::clone(&sd)));
        });
    }

    pub async fn open_project_dialog(sd: Rc<RefCell<Self>>) {
        let folder_dialog = gtk::FileChooserDialog::builder()
            .title("Open project folder...")
            .action(gtk::FileChooserAction::SelectFolder)
            .build();
        folder_dialog.add_button("Open Folder", gtk::ResponseType::Accept);
        folder_dialog.add_button("Cancel", gtk::ResponseType::Cancel);

        let folder_path = folder_dialog.run_future().await;
        match folder_path {
            gtk::ResponseType::Accept => {
                let mut sdm = sd.borrow_mut();
                let path = String::from(folder_dialog.filename().unwrap().to_string_lossy());
                sdm.state.project = Some(Project {
                    title: path.clone(),
                    path: path.clone(),
                    docs: HashMap::new(),
                });
                sdm.window.imp().headerbar.set_subtitle(Some(&path));
            }
            gtk::ResponseType::Cancel => println!("Nevermind!"),
            _ => unreachable!(),
        }
        folder_dialog.close();
    }
}
