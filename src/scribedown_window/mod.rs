use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use gtk::glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio, glib};

mod imp;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Window {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create ScribeDownWindow")
    }
}