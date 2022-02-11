use gio::subclass::prelude::*;
use gtk::{gio, glib, prelude::*};

use std::cell::RefCell;

use crate::document_list::document_list_row_data::RowData;

#[derive(Debug, Default)]
pub struct Model(pub(super) RefCell<Vec<RowData>>);

#[glib::object_subclass]
impl ObjectSubclass for Model {
    const NAME: &'static str = "DocumentListModel";
    type Type = super::Model;
    type Interfaces = (gio::ListModel,);
}

impl ObjectImpl for Model {}

impl ListModelImpl for Model {
    fn item_type(&self, _list_model: &Self::Type) -> glib::Type {
        RowData::static_type()
    }
    fn n_items(&self, _list_model: &Self::Type) -> u32 {
        self.0.borrow().len() as u32
    }
    fn item(&self, _list_model: &Self::Type, position: u32) -> Option<glib::Object> {
        self.0
            .borrow()
            .get(position as usize)
            .map(|o| o.clone().upcast::<glib::Object>())
    }
}