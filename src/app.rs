use crate::document_list;
use crate::file_operations;
use crate::macro_utils::*;
use gtk::TextBuffer;
use gtk::{glib::Value, prelude::*, subclass::prelude::*};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A ScribeDown project
#[derive(Clone)]
pub struct Project {
    /// Folder path
    pub path: String,
    /// Documents, indexed by their path
    pub docs: HashMap<String, Document>,
}

/// Represents a single open document
#[derive(Clone)]
pub struct Document {
    /// Document location
    pub path: String,
    /// Document name
    pub title: String,
    /// Document plaintext markdown contents
    pub contents: Option<String>,
}

/// Application state
#[derive(Clone)]
pub struct State {
    /// Current open project
    pub project: Option<Project>,

    /// What files are open in the various tabs (referenced by path into the `project.docs`)
    pub open_files: Vec<String>,
}

/// Central storage for UI and app state
pub struct App {
    pub window: crate::scribedown_window::Window,
    pub state: State,
    pub document_list_model: Option<document_list::Model>,
}

impl App {
    /// Connect all callbacks
    pub fn connect_all(sd: Rc<RefCell<Self>>) {
        // NOTE: the outer pointers to `sd`, formatted like `sd_for_*`, are
        // done in order to prevent the callback from borrowing the original
        // pointer when it creates its own pointer, which we need to keep free
        // to continue making more pointers. This happens because using
        // something inside a `move` callback borrows it.

        // Connect open button callback
        let sd_for_button = sd.clone();
        {
            let osd = sd.borrow();
            let button = &osd.window.imp().open_button;

            button.connect_clicked(move |_| {
                // Launch dialog in new thread so it doesn't hang this one
                gtk::glib::MainContext::default()
                    .spawn_local(Self::open_project_dialog(Rc::clone(&sd_for_button)));
            });
        }

        // Put the contents of the currently selected document in the text
        // editor and switch the notebook tab to the correct tab
        let sd_for_list = sd.clone();
        {
            let osd = sd.borrow();
            let document_list = &osd.window.imp().document_list;

            document_list.connect_row_selected(move |_dl, row| {
                println!("Row selected!");

                // Get a pointer to the state to use for this callback in
                // perpetuity, and borrow it.
                let sd3 = Rc::clone(&sd_for_list);
                let osd = sd3.borrow();

                // Get the GUI list box row that was just selected
                let row = unwrap_or_return!(row);
                let list_box_row =
                    unwrap_or_return!(row.downcast_ref::<document_list::ListBoxRow>());

                // Get the row data associated with that GUI element
                let row_data_val: Value = list_box_row.property("row-data");
                let row_data = unwrap_ok_or_return!(row_data_val.get::<document_list::RowData>());
                let path = row_data.property::<String>("path");
                println!("Path: {:?}", path);

                // Get document that that row data points to from the current project
                let project = unwrap_or_return!(&osd.state.project);
                let doc = unwrap_or_return!(project.docs.get(&path));
                println!("Document title: {:?}", doc.title);

                // Update the text editor
                let text_buffer = TextBuffer::builder()
                    .text(doc.contents.clone().unwrap().as_str())
                    .build();

                osd.window.imp().text_editor.set_buffer(Some(&text_buffer));
            });
        }
    }

    /// Open a folder chooser dialog which, when a folder is selected:
    /// 1. creates a new project with that path
    /// 2. sets the headerbar's subtitle to that path.
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
                let path = String::from(folder_dialog.filename().unwrap().to_string_lossy());
                App::open_project(sd, path);
            }
            _ => println!("Nevermind!"),
        }
        folder_dialog.close();
    }

    pub fn open_project(sd: Rc<RefCell<Self>>, path: String) {
        let mut sdm = sd.borrow_mut();
        let docs = file_operations::get_md_files(path.clone());

        // Update back-end state
        sdm.state.project = Some(Project {
            path: path.clone(),
            docs: docs.clone(),
        });

        // Update UI
        let imp = &sdm.window.imp();
        imp.headerbar.set_subtitle(Some(&path));

        // Update document list model
        let dlm = document_list::Model::new();
        for (path, doc) in docs {
            let rd = document_list::RowData::new(&path, &doc.title);
            dlm.append(&rd);
        }
        imp.document_list.bind_model(Some(&dlm), move |item| {
            document_list::ListBoxRow::new(
                item.downcast_ref::<document_list::RowData>()
                    .expect("RowData is of wrong type"),
            )
            .upcast::<gtk::Widget>()
        });
    }
}
