use crate::app::AppState;
use crate::geometry::{Point, Rect};
use crate::html::AddListener;
use crate::{handle_touch_end, handle_touch_move, handle_touch_start};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{HtmlCanvasElement, MouseEvent, TouchEvent};

pub trait StateSubscriber {
    fn subscribe(&self, app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue>;
}

impl StateSubscriber for HtmlCanvasElement {
    fn subscribe(&self, app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        let canvas_rect: Rect = self.get_bounding_client_rect().into();
        let adjust_location = move |pos: Point| -> Point {
            Point {
                x: pos.x - canvas_rect.x,
                y: pos.y - canvas_rect.y,
            }
        };

        {
            let app_state = app_state.clone();
            self.add_listener("mousedown", move |event: MouseEvent| {
                let mut app_state = app_state.borrow_mut();
                handle_touch_start(app_state.drawing_expected_mut(), Some(event.into()))
            })?
        }
        {
            let app_state = app_state.clone();
            self.add_listener("mousemove", move |event: MouseEvent| {
                let mut app_state = app_state.borrow_mut();
                handle_touch_move(app_state.drawing_expected_mut(), event.into())
            })?
        }
        {
            let app_state = app_state.clone();
            self.add_listener("mouseup", move |event: MouseEvent| {
                let mut app_state = app_state.borrow_mut();
                handle_touch_end(app_state.drawing_expected_mut(), Some(event.into()))
            })?
        }
        {
            let app_state = app_state.clone();
            self.add_listener("touchstart", move |event: TouchEvent| {
                let point = event.try_into().ok().map(adjust_location);
                let mut app_state = app_state.borrow_mut();
                handle_touch_start(app_state.drawing_expected_mut(), point)
            })?
        }
        {
            let app_state = app_state.clone();
            self.add_listener("touchmove", move |event: TouchEvent| {
                let point = event.try_into().ok().map(adjust_location);
                if let Some(point) = point {
                    let mut app_state = app_state.borrow_mut();
                    handle_touch_move(app_state.drawing_expected_mut(), point)
                }
            })?
        }
        {
            let app_state = app_state.clone();
            self.add_listener("touchend", move |event: TouchEvent| {
                let point = event.try_into().ok().map(adjust_location);
                let mut app_state = app_state.borrow_mut();
                handle_touch_end(app_state.drawing_expected_mut(), point)
            })?
        }

        Ok(())
    }
}
