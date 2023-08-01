use crate::app_state::{AppState, DrawingState, InitialState};
use crate::handle_advance_btn_click;
use crate::html::HtmlDom;
use crate::subscribe_state::StateSubscriber;
use commons::geometry::Point2D;
use drawing_commons::models::DrawingPaths;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::subscribers::AddListener;
use web_sys::MouseEvent;

trait SubscribeDrawings {
    fn subscribe_sketch_pad_events(
        &self,
        app_state: Rc<RefCell<AppState<HtmlDom>>>,
    ) -> Result<(), JsValue>;
}

impl SubscribeDrawings for DrawingState<HtmlDom> {
    fn subscribe_sketch_pad_events(
        &self,
        app_state: Rc<RefCell<AppState<HtmlDom>>>,
    ) -> Result<(), JsValue> {
        let on_update_callback = Rc::new(RefCell::new(move |drawing: &DrawingPaths<Point2D>| {
            let mut app_state = app_state.borrow_mut();
            let app_state = app_state.drawing_expected_mut().expect("");
            *app_state.curr_path_mut() = drawing.clone();
        }));

        self.get_view()
            .sketch_pad
            .borrow_mut()
            .set_on_update(on_update_callback);

        Ok(())
    }
}

impl StateSubscriber for DrawingState<HtmlDom> {
    fn subscribe(&self, app_state: Rc<RefCell<AppState<HtmlDom>>>) -> Result<(), JsValue> {
        self.subscribe_sketch_pad_events(app_state.clone())
    }
}

impl StateSubscriber for InitialState<HtmlDom> {
    fn subscribe(&self, app_state: Rc<RefCell<AppState<HtmlDom>>>) -> Result<(), JsValue> {
        self.get_view()
            .advance_btn
            .on_click(move |_event: MouseEvent| handle_advance_btn_click(&app_state))
    }
}

impl StateSubscriber for AppState<HtmlDom> {
    fn subscribe(&self, app_state: Rc<RefCell<AppState<HtmlDom>>>) -> Result<(), JsValue> {
        match self {
            AppState::Initial(state) => state.subscribe(app_state.clone()),
            AppState::Drawing(state) => state.subscribe(app_state),
            AppState::Ready(_) => Ok(()),
            AppState::Saved(_) => Ok(()),
        }
    }
}
