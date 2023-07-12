use crate::app::AppState;
use crate::geometry::{Point, Rect};
use crate::html::AddListener;
use crate::{handle_touch_end, handle_touch_move, handle_touch_start};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{MouseEvent, TouchEvent};

pub fn subscribe_canvas_events(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let canvas = app_state.borrow().get_html_dom().canvas.clone();
    let canvas_rect: Rect = canvas.get_bounding_client_rect().into();
    let adjust_location = move |pos: Point| -> Point {
        Point {
            x: pos.x - canvas_rect.x,
            y: pos.y - canvas_rect.y,
        }
    };

    {
        let app_state = app_state.clone();
        canvas.add_listener("mousedown", move |event: MouseEvent| {
            handle_touch_start(&mut app_state.borrow_mut(), Some(event.into()))
        })?
    }
    {
        let app_state = app_state.clone();
        canvas.add_listener("mousemove", move |event: MouseEvent| {
            handle_touch_move(&app_state, event.into()).unwrap()
        })?
    }
    {
        let app_state = app_state.clone();
        canvas.add_listener("mouseup", move |event: MouseEvent| {
            handle_touch_end(&app_state, Some(event.into())).unwrap()
        })?
    }
    {
        let app_state = app_state.clone();
        canvas.add_listener("touchstart", move |event: TouchEvent| {
            let point = event.try_into().ok().map(adjust_location);
            handle_touch_start(&mut app_state.borrow_mut(), point)
        })?
    }
    {
        let app_state = app_state.clone();
        canvas.add_listener("touchmove", move |event: TouchEvent| {
            let point = event.try_into().ok().map(adjust_location);
            if let Some(point) = point {
                handle_touch_move(&app_state, point).unwrap()
            }
        })?
    }
    {
        let app_state = app_state.clone();
        canvas.add_listener("touchend", move |event: TouchEvent| {
            let point = event.try_into().ok().map(adjust_location);
            handle_touch_end(&app_state, point).unwrap()
        })?
    }

    Ok(())
}
