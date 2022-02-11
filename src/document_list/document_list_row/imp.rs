use std::cell::RefCell;

use gtk::{
    glib::{self, ParamSpec, ParamSpecObject, Value},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

use crate::document_list::document_list_row_data::RowData;

#[derive(Default, Debug, CompositeTemplate)]
#[template(file = "document_list_row.ui")]
pub struct ListBoxRow {
    /// UI elements we need access to
    #[template_child]
    pub title_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub path_label: TemplateChild<gtk::Label>,
    /// Internal state
    row_data: RefCell<Option<RowData>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ListBoxRow {
    const NAME: &'static str = "DocumentListListBoxRow";
    type ParentType = gtk::ListBoxRow;
    type Type = super::ListBoxRow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass)
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ListBoxRow {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "row-data",
                "Row Data",
                "Row Data",
                RowData::static_type(),
                glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "row-data" => {
                let row_data = value.get().unwrap();
                self.row_data.replace(row_data);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "row-data" => self.row_data.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let item = self.row_data.borrow();
        let item = item.as_ref().cloned().unwrap();
        item.bind_property("path", &self.path_label.get(), "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        item.bind_property("title", &self.title_label.get(), "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
}

impl WidgetImpl for ListBoxRow {}
impl ContainerImpl for ListBoxRow {}
impl BinImpl for ListBoxRow {}
impl ListBoxRowImpl for ListBoxRow {}
