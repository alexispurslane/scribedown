use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct MarkdownEditor(ObjectSubclass<imp::MarkdownEditor>)
        @extends gtk::ScrolledWindow, gtk::Container, gtk::Widget,
        @implements gtk::Buildable, gtk::Scrollable;
}

impl MarkdownEditor {
    pub fn new(contents: &String) -> Self {
        println!("{:?}", contents);
        glib::Object::new(&[("contents", contents)]).expect("Failed to create MarkdownEditor")
    }
}
