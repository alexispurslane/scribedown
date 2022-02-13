use gtk::{glib::Object, prelude::*, Bin, Container, Widget};

/// Returns the child element which has the given name.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Button, ContainerExt, WidgetExt, Window, WindowType};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let but = Button::new();
/// let w = Window::new(WindowType::Toplevel);
///
/// but.set_name("Button");
/// w.add(&but);
///
/// gtk_test::find_child_by_name::<Button, Window>(&w, "Button").expect("failed to find child");
/// // Or even better:
/// let but: Button = gtk_test::find_child_by_name(&w, "Button").expect("failed to find child");
/// # }
/// ```
pub fn find_child_by_name<C: IsA<Widget>, W: Clone + IsA<Object> + IsA<Widget>>(
    parent: &W,
    name: &str,
) -> Option<C> {
    find_widget_by_name(parent, name).and_then(|widget| widget.downcast().ok())
}

/// Returns the child widget which has the given name.
///
/// Example:
///
/// ```
/// extern crate gtk;
/// #[macro_use]
/// extern crate gtk_test;
///
/// use gtk::{Button, ContainerExt, WidgetExt, Window, WindowType};
///
/// # fn main() {
/// gtk::init().expect("GTK init failed");
/// let but = Button::new();
/// let w = Window::new(WindowType::Toplevel);
///
/// but.set_name("Button");
/// w.add(&but);
///
/// gtk_test::find_widget_by_name(&w, "Button").unwrap();
/// # }
/// ```
pub fn find_widget_by_name<W: Clone + IsA<Object> + IsA<Widget>>(
    parent: &W,
    name: &str,
) -> Option<Widget> {
    if let Ok(container) = parent.clone().dynamic_cast::<Container>() {
        for child in container.children() {
            let string = String::from(child.widget_name().as_str());
            if string == name {
                return Some(child);
            }
            if let Some(widget) = find_widget_by_name(&child, name) {
                return Some(widget);
            }
        }
    } else if let Ok(bin) = parent.clone().dynamic_cast::<Bin>() {
        if let Some(child) = bin.child() {
            let string = String::from(child.widget_name().as_str());
            if string == name {
                return Some(child);
            }
            if let Some(widget) = find_widget_by_name(&child, name) {
                return Some(widget);
            }
        }
    }
    None
}
