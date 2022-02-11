mod imp;

use gtk::glib;

// Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
// binding
glib::wrapper! {
    pub struct RowData(ObjectSubclass<imp::RowData>);
}

// Constructor for new instances. This simply calls glib::Object::new() with
// initial values for our two properties and then returns the new instance
impl RowData {
    pub fn new(path: &str, title: &str) -> RowData {
        glib::Object::new(&[("path", &path), ("title", &title)]).expect("Failed to create row data")
    }
}