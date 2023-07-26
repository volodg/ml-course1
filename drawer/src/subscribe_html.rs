use crate::app_state::{AppState, DrawingState, InitialState};
use crate::draw::Draw;
use crate::geometry::Rect;
use crate::html::HtmlDom;
use crate::subscribe_state::StateSubscriber;
use crate::{handle_advance_btn_click, handle_touch_end, handle_touch_move, handle_touch_start};
use commons::math::Point;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_commons::geometry::{convert_mouse_event_into_point, try_convert_touch_event_into_point};
use web_commons::html::AddListener;
use web_sys::{MouseEvent, TouchEvent};

trait SubscribeDrawings {
    fn subscribe_canvas_events(
        &self,
        app_state: Rc<RefCell<AppState<HtmlDom>>>,
    ) -> Result<(), JsValue>;

    fn subscribe_to_undo_btn(
        &self,
        app_state: Rc<RefCell<AppState<HtmlDom>>>,
    ) -> Result<(), JsValue>;
}

impl SubscribeDrawings for DrawingState<HtmlDom> {
    fn subscribe_canvas_events(
        &self,
        app_state: Rc<RefCell<AppState<HtmlDom>>>,
    ) -> Result<(), JsValue> {
        let view = self.get_view();
        let canvas_rect: Rect = view.canvas.get_bounding_client_rect().into();
        let adjust_location = move |pos: Point| -> Point {
            Point {
                x: pos.x - canvas_rect.x as f64,
                y: pos.y - canvas_rect.y as f64,
            }
        };

        {
            let app_state = app_state.clone();
            view.canvas
                .add_listener("mousedown", move |event: MouseEvent| {
                    let mut app_state = app_state.borrow_mut();
                    handle_touch_start(
                        app_state.drawing_expected_mut().expect(""),
                        Some(convert_mouse_event_into_point(event)),
                    )
                })?
        }
        {
            let app_state = app_state.clone();
            view.canvas
                .add_listener("mousemove", move |event: MouseEvent| {
                    let mut app_state = app_state.borrow_mut();
                    handle_touch_move(
                        app_state.drawing_expected_mut().expect(""),
                        convert_mouse_event_into_point(event),
                    )
                })?
        }
        {
            let app_state = app_state.clone();
            view.document
                .add_listener("mouseup", move |_event: MouseEvent| {
                    let mut app_state = app_state.borrow_mut();
                    if let Some(state) = app_state.drawing_expected_mut() {
                        handle_touch_end(state)
                    }
                })?
        }
        {
            let app_state = app_state.clone();
            view.canvas
                .add_listener("touchstart", move |event: TouchEvent| {
                    let point = try_convert_touch_event_into_point(event)
                        .ok()
                        .map(adjust_location);
                    let mut app_state = app_state.borrow_mut();
                    handle_touch_start(app_state.drawing_expected_mut().expect(""), point)
                })?
        }
        {
            let app_state = app_state.clone();
            view.canvas
                .add_listener("touchmove", move |event: TouchEvent| {
                    let point = try_convert_touch_event_into_point(event)
                        .ok()
                        .map(adjust_location);
                    if let Some(point) = point {
                        let mut app_state = app_state.borrow_mut();
                        handle_touch_move(app_state.drawing_expected_mut().expect(""), point)
                    }
                })?
        }
        {
            view.document
                .add_listener("touchend", move |_event: TouchEvent| {
                    let mut app_state = app_state.borrow_mut();
                    if let Some(state) = app_state.drawing_expected_mut() {
                        handle_touch_end(state)
                    }
                })?
        }

        Ok(())
    }

    fn subscribe_to_undo_btn(
        &self,
        app_state: Rc<RefCell<AppState<HtmlDom>>>,
    ) -> Result<(), JsValue> {
        self.get_view()
            .undo_btn
            .on_click(move |_event: MouseEvent| {
                let mut app_state = app_state.borrow_mut();
                let state = app_state.drawing_expected_mut().expect("");
                state.undo();
                state.draw().unwrap()
            })
    }
}

impl StateSubscriber for DrawingState<HtmlDom> {
    fn subscribe(&self, app_state: Rc<RefCell<AppState<HtmlDom>>>) -> Result<(), JsValue> {
        self.subscribe_canvas_events(app_state.clone())?;
        self.subscribe_to_undo_btn(app_state)
    }
}

impl StateSubscriber for InitialState<HtmlDom> {
    fn subscribe(&self, app_state: Rc<RefCell<AppState<HtmlDom>>>) -> Result<(), JsValue> {
        self.get_view()
            .advance_btn
            .on_click(move |_event: MouseEvent| handle_advance_btn_click(&app_state).unwrap())
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
