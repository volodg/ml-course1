use web_sys::HtmlElement;

pub trait Visibility {
    fn set_visible(&self, visible: bool);
    fn set_display(&self, visible: bool);
}

impl Visibility for HtmlElement {
    fn set_visible(&self, visible: bool) {
        if visible {
            self.style().set_property("visibility", "visible").unwrap();
        } else {
            self.style().set_property("visibility", "hidden").unwrap();
        }
    }

    fn set_display(&self, display: bool) {
        if display {
            self.style().remove_property("display").unwrap();
        } else {
            self.style().set_property("display", "none").unwrap();
        }
    }
}
