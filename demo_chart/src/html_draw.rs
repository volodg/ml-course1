use crate::app_state::AppState;
use crate::draw::DrawWithState;
use crate::html::HtmlDom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::chart_models::Sample;
use web_commons::document::DocumentExt;
use web_commons::html::AddListener;
use web_sys::{
    window, HtmlTableRowElement, HtmlTableSectionElement, ScrollBehavior, ScrollIntoViewOptions,
    ScrollLogicalPosition,
};

impl DrawWithState for HtmlDom {
    fn draw(&self, app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        let header = self
            .data_table
            .create_t_head()
            .dyn_into::<HtmlTableSectionElement>()?;
        let tr = header.insert_row()?.dyn_into::<HtmlTableRowElement>()?;
        tr.insert_cell()?.set_inner_html("Id");
        tr.insert_cell()?.set_inner_html("Type");
        tr.insert_cell()?.set_inner_html("Km");
        tr.insert_cell()?.set_inner_html("Price");

        let body = self
            .data_table
            .create_t_body()
            .dyn_into::<HtmlTableSectionElement>()?;

        for sample in &app_state.borrow().samples {
            let tr = body.insert_row()?.dyn_into::<HtmlTableRowElement>()?;
            tr.set_id(sample.element_id().as_str());
            {
                let sample = sample.clone();
                let app_state = app_state.clone();
                tr.on_click(move |_| {
                    app_state
                        .borrow()
                        .html
                        .handle_click(Some(&sample), false)
                        .expect("");
                })?;
            }
            tr.insert_cell()?
                .set_inner_html(sample.id.to_string().as_str());
            tr.insert_cell()?.set_inner_html(sample.label.as_str());
            tr.insert_cell()?
                .set_inner_html(std::format!("{:.0}", sample.point.x).as_str());
            tr.insert_cell()?
                .set_inner_html(std::format!("{:.0}", sample.point.y).as_str());
        }

        let callback_app_state = app_state.clone();

        let on_click_callback = Rc::new(RefCell::new(move |sample: Option<&Sample>| {
            callback_app_state
                .borrow()
                .html
                .handle_click(sample, true)
                .expect("");
        }));

        let mut chart = self.chart.borrow_mut();
        chart.set_samples(app_state.borrow().samples.clone());
        chart.set_on_click(on_click_callback);

        chart.draw()
    }
}

trait HtmlDomExt {
    fn handle_click(&self, sample: Option<&Sample>, scroll: bool) -> Result<(), JsValue>;
}

impl HtmlDomExt for HtmlDom {
    fn handle_click(&self, sample: Option<&Sample>, scroll: bool) -> Result<(), JsValue> {
        let document = window().expect("").document().expect("");

        let emphasize_class_name = "emphasize";
        let de_emphasize =
            || -> Result<(), JsValue> { document.remove_all_classes(emphasize_class_name) };

        match sample {
            None => de_emphasize()?,
            Some(sample) => {
                let element = document
                    .get_element_by_id(sample.element_id().as_str())
                    .expect("");

                if element.class_list().contains(emphasize_class_name) {
                    element.class_list().remove_1(emphasize_class_name)?;
                } else {
                    de_emphasize()?;

                    element.class_list().add_1(emphasize_class_name)?;

                    if scroll {
                        let mut options = ScrollIntoViewOptions::new();
                        options.behavior(ScrollBehavior::Auto);
                        options.block(ScrollLogicalPosition::Center);
                        element.scroll_into_view_with_scroll_into_view_options(&options);
                    }
                }

                self.chart.borrow_mut().select_sample(Some(sample))?;
            }
        }

        Ok(())
    }
}

trait SampleExt {
    fn element_id(&self) -> String;
}

impl SampleExt for Sample {
    fn element_id(&self) -> String {
        std::format!("sample_{}", self.id)
    }
}
