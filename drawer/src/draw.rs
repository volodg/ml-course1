use crate::app_state::{DrawingState, ReadyState, SavedState};
use crate::html::HtmlDom;
use wasm_bindgen::JsValue;
use web_commons::html::Visibility;

pub trait Draw {
    fn draw(&self) -> Result<(), JsValue>;
}

impl Draw for DrawingState<HtmlDom> {
    fn draw(&self) -> Result<(), JsValue> {
        let view = self.get_view();
        view.sketch_pad.borrow().set_visible(true)?;

        view.student_input.set_display(false)?;
        view.instructions_spn.set_display(true)?;
        view.advance_btn.set_inner_html("NEXT");

        let label = self.get_current_label();
        view.instructions_spn
            .set_inner_html(std::format!("Please draw a {label}").as_str());
        Ok(())
    }
}

impl Draw for ReadyState<HtmlDom> {
    fn draw(&self) -> Result<(), JsValue> {
        let view = self.get_view();
        view.sketch_pad.borrow().set_visible(false)?;

        view.instructions_spn.set_inner_html("Thank you!");
        view.advance_btn.set_inner_html("SAVE");
        Ok(())
    }
}

impl Draw for SavedState<HtmlDom> {
    fn draw(&self) -> Result<(), JsValue> {
        let view = self.get_view();
        view.advance_btn.set_display(false)?;
        view.instructions_spn.set_inner_html(
            "Take you downloaded file and place it along side the others in the dataset!",
        );
        Ok(())
    }
}
