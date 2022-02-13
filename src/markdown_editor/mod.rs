use crate::app::Document;
use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};
use std::cell::RefCell;

mod imp;

glib::wrapper! {
    pub struct MarkdownEditor(ObjectSubclass<imp::MarkdownEditor>)
        @extends gtk::ScrolledWindow, gtk::Container, gtk::Widget,
        @implements gtk::Buildable, gtk::Scrollable;
}

impl MarkdownEditor {
    pub fn new(path: &str, title: &str, contents: &str) -> Self {
        glib::Object::new(&[("path", &path), ("title", &title), ("contents", &contents)])
            .expect("Failed to create MarkdownEditor")
    }
}
