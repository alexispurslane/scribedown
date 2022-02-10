use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk::glib::clone;
use gtk::{prelude::*, TextBuffer};
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

#[derive(CompositeTemplate, Default)]
#[template(file = "window.ui")]
pub struct Window {
    // Side panel
    #[template_child]
    pub document_list: TemplateChild<gtk::ListBox>,
    #[template_child]
    pub document_search: TemplateChild<gtk::SearchEntry>,

    // Main panel
    #[template_child]
    pub editor_notebook: TemplateChild<gtk::Notebook>,
    #[template_child]
    pub text_editor: TemplateChild<gtk::TextView>,

    // Header bar
    #[template_child]
    pub headerbar: TemplateChild<gtk::HeaderBar>,
    #[template_child]
    pub open_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub new_file_button: TemplateChild<gtk::Button>,

    // Popover menu
    #[template_child]
    pub save_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub save_as_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub undo_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub redo_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub preferences_button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "ScribeDownWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}

impl WidgetImpl for Window {}
impl ContainerImpl for Window {}
impl BinImpl for Window {}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
