use glib::subclass::prelude::*;
use gtk::{
    glib::{self, ParamSpec, Value},
    prelude::*,
};
use std::cell::RefCell;

#[derive(Default)]
pub struct RowData {
    path: RefCell<String>,
    title: RefCell<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for RowData {
    const NAME: &'static str = "DocumentListRowData";
    type Type = super::RowData;
}

impl ObjectImpl for RowData {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::new(
                    "path",
                    "Path",
                    "Path",
                    None,
                    glib::ParamFlags::READWRITE,
                ),
                glib::ParamSpecString::new(
                    "title",
                    "Title",
                    "Title",
                    Some("Untitled"),
                    glib::ParamFlags::READWRITE,
                ),
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "path" => {
                let path = value.get().unwrap();
                self.path.replace(path);
            }
            "title" => {
                let title = value.get().unwrap();
                self.title.replace(title);
            }
            _ => unimplemented!()
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "path" => self.path.borrow().to_value(),
            "title" => self.title.borrow().to_value(),
            _ => unimplemented!()
        }
    }
}
