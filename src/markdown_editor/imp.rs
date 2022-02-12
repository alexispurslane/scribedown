use std::cell::RefCell;

use gtk::{
    glib::{self, ParamSpec, ParamSpecObject, Value},
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
};

#[derive(Default, Debug, CompositeTemplate)]
#[template(file = "markdown_editor.ui")]
pub struct MarkdownEditor {
    /// UI elements we need access to
    #[template_child]
    pub text_editor: TemplateChild<gtk::TextView>,
    // Internal state
    contents: RefCell<String>,
    text_buffer: RefCell<Option<gtk::TextBuffer>>,
    // probably going to also need to store a RefCell to:
    // 1. the parsed markdown
    // 2. the undo tree
    // 3. the ranges from (1) for styling
}

#[glib::object_subclass]
impl ObjectSubclass for MarkdownEditor {
    const NAME: &'static str = "MarkdownEditor";
    type Type = super::MarkdownEditor;
    type ParentType = gtk::ScrolledWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass)
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for MarkdownEditor {
    fn properties() -> &'static [ParamSpec] {
        use once_cell::sync::Lazy;
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "contents",
                "Contents",
                "Contents",
                String::static_type(),
                glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
            )]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "contents" => {
                let contents = value.get().unwrap();
                self.contents.replace(contents);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "contents" => self.contents.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let tb = gtk::TextBuffer::builder()
            .text(&self.contents.borrow())
            .build();
        self.text_editor.set_buffer(Some(&tb));
        self.text_buffer.replace(Some(tb));
    }
}

impl WidgetImpl for MarkdownEditor {}
impl ContainerImpl for MarkdownEditor {}
impl BinImpl for MarkdownEditor {}
impl ScrolledWindowImpl for MarkdownEditor {}
