use crate::document_list;
use crate::file_operations;
use crate::macro_utils::*;
use crate::markdown_editor;
use gtk::gdk;
use gtk::{
    glib::{Receiver, Sender, Value},
    prelude::*,
    subclass::prelude::*,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A ScribeDown project
pub struct Project {
    /// Folder path
    pub path: String,
    /// Documents, indexed by their path
    pub docs: HashMap<String, Document>,
    /// Keeps a list of open tabs by document title, and their index in the notebook
    pub tabs: HashMap<String, u32>,
}

/// Represents a single open document
#[derive(Clone)]
pub struct Document {
    /// Document location
    pub path: String,
    /// Document name
    pub title: String,
}

/// Application state
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

enum AppKeyEvent {
    Undo,
    Redo,
    Save,
    CloseTab,
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
        {
            let sd_for_button = sd.clone();
            let osd = sd.borrow();
            let button = &osd.window.imp().open_button;

            button.connect_clicked(move |_| {
                // Launch dialog in new thread so it doesn't hang this one
                gtk::glib::MainContext::default()
                    .spawn_local(Self::open_project_dialog(Rc::clone(&sd_for_button)));
            });
        }

        // Connect row selection callback for document list
        {
            let sd_for_list = sd.clone();
            let osd = sd.borrow();
            let document_list = &osd.window.imp().document_list;

            document_list.connect_row_selected(move |_dl, row| {
                println!("Row selected!");
                // Get a pointer to the state to use for this callback in
                // perpetuity
                Self::open_document(row, Rc::clone(&sd_for_list));
            });
        }

        // Connect key command callback for app
        {
            let osd = sd.borrow();
            let (tx, rx): (Sender<AppKeyEvent>, Receiver<AppKeyEvent>) =
                gtk::glib::MainContext::channel(gtk::glib::PRIORITY_DEFAULT);
            let _ = &osd.window.connect("key_press_event", false, move |values| {
                let raw_event = &values[1].get::<gdk::Event>().unwrap();
                match raw_event.downcast_ref::<gdk::EventKey>() {
                    Some(event) => {
                        if event.state().contains(gdk::ModifierType::CONTROL_MASK) {
                            match event.keyval().to_unicode().unwrap_or('\0') {
                                's' => {
                                    tx.send(AppKeyEvent::Save).unwrap();
                                }
                                'w' => {
                                    tx.send(AppKeyEvent::CloseTab).unwrap();
                                }
                                'z' => {
                                    tx.send(AppKeyEvent::Undo).unwrap();
                                }
                                'Y' => {
                                    tx.send(AppKeyEvent::Redo).unwrap();
                                }
                                x => println!("{:?}", x),
                            }
                        }
                    }
                    None => {}
                }
                let result = (false).to_value();
                Some(result)
            });
            let sd_for_key_event = sd.clone();
            rx.attach(None, move |evt: AppKeyEvent| {
                match evt {
                    AppKeyEvent::CloseTab => {
                        let sd = sd_for_key_event.borrow();
                        let notebook = &sd.window.imp().editor_notebook;
                        notebook.remove_page(notebook.current_page());
                    }
                    AppKeyEvent::Save => {
                        Self::save_document(Rc::clone(&sd_for_key_event));
                    }
                    _ => {}
                }
                Continue(true)
            });
        }
    }

    /* ---------------------- GUI HELPERS ---------------------- */

    /// Open a folder chooser dialog that calls `open_project`
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

    /* ------------------------ ACTIONS ------------------------ */

    /// Put the contents of the currently selected document in the text
    /// editor and switch the notebook tab to the correct tab (or add a tab if
    /// its a new document)
    fn open_document(row: Option<&gtk::ListBoxRow>, sd3: Rc<RefCell<Self>>) {
        // Borrow state stored in pointer
        let mut osd = sd3.borrow_mut();
        // NOTE: This is probably dangerous, but as long as we only need to
        // mutate `osd.state`, and read from `osd.window.imp()`, it should be
        // fine!
        let osd = &mut *osd;

        // Get the GUI list box row that was just selected
        let row = unwrap_or_return!(row);
        let list_box_row = unwrap_or_return!(row.downcast_ref::<document_list::ListBoxRow>());

        // Get the row data associated with that GUI element
        let row_data_val: Value = list_box_row.property("row-data");
        let row_data = unwrap_ok_or_return!(row_data_val.get::<document_list::RowData>());
        let title = row_data.property::<String>("title");
        println!("Title: {:?}", title);

        // Get document that that row data points to from the current project
        let project = unwrap_or_return!(&mut osd.state.project);
        let doc = unwrap_or_return!(project.docs.get(&title));
        println!("Document title: {:?}", doc.title);
        let window = &osd.window.imp();
        let notebook = &window.editor_notebook;

        if let Some(page_num) = project.tabs.get(&doc.title) {
            notebook.set_current_page(Some(*page_num));
        } else {
            // Open document in new notebook tab

            // Create a new text editor textview with those contents
            // FIXME: Load contents from file
            let contents = file_operations::get_file_contents(&doc.path).expect("Cannot open file");
            let text_editor =
                markdown_editor::MarkdownEditor::new(&doc.path, &doc.title, &contents);
            let scrolled = gtk::ScrolledWindow::builder().child(&text_editor).build();
            scrolled.show_all();

            let tab_label = gtk::Label::new(Some(doc.title.as_str()));
            tab_label.show();

            // open that text editor in a new tab
            println!("Appending page\n");
            let page_num = notebook.append_page(&scrolled, Some(&tab_label));
            notebook.set_current_page(Some(page_num));
            project.tabs.insert(doc.title.clone(), page_num);
        }
    }

    /// Saves the contents of the text editor in the current tab to disk and
    /// the back end project document model
    pub fn save_document(sd: Rc<RefCell<Self>>) {
        let sd = sd.borrow();
        let notebook = &sd.window.imp().editor_notebook;
        let page_num = notebook.current_page();

        // Get text editor for tab
        let raw_te = notebook
            .nth_page(page_num)
            .expect("Could not get current notebook page");
        let sw = raw_te
            .downcast::<gtk::ScrolledWindow>()
            .expect("Current notebook page not a markdown editor?!");
        let vp = sw
            .child()
            .expect("Scrollable window needs child!")
            .downcast::<gtk::Viewport>()
            .expect("Scrollable window needs Viewport child, instead got: ");
        let markdown_editor = vp
            .child()
            .expect("Viewport window needs child!")
            .downcast::<markdown_editor::MarkdownEditor>()
            .expect("Viewport needs TextView child");

        let path = markdown_editor.property::<String>("path");
        // Get text editor contents
        let buffer = markdown_editor.imp().text_editor.get().buffer().unwrap();
        let new_contents = buffer.text(&buffer.start_iter(), &buffer.end_iter(), false);
        let new_contents = new_contents
            .map(|x| String::from(x.as_str()))
            .expect("GString should convert to String");

        // TODO: Write new doc contents to file
        file_operations::save_file_contents(&path, new_contents);
    }

    /// Update the back end state to point to a new project with the proper
    /// path and doc list. Then update the headerbar and create a document list
    /// model out of the doc list for the side bar to use.
    pub fn open_project(sd: Rc<RefCell<Self>>, path: String) {
        let mut sdm = sd.borrow_mut();
        let docs = file_operations::get_md_files(path.clone());

        // Update back-end state
        sdm.state.project = Some(Project {
            path: path.clone(),
            docs: docs.clone(),
            tabs: HashMap::new(),
        });

        // Update UI
        let imp = &sdm.window.imp();
        imp.headerbar.set_subtitle(Some(&path));

        // Update document list model
        let dlm = document_list::Model::new();
        for (title, doc) in docs {
            let rd = document_list::RowData::new(&doc.path, &title);
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
