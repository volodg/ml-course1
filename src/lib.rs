mod app;
mod canvas;
mod geometry;
mod html;

use crate::app::{AppState, DrawingState, ReadyState, SavedState};
use crate::geometry::Point;
use crate::html::{alert, AddListener, HtmlDom};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn handle_next(app_state: &Rc<RefCell<AppState>>) {
    let new_state = {
        let mut app_state = app_state.borrow_mut();
        let state = app_state.drawing_expected_mut();
        if state.curr_path().is_empty() {
            alert("Draw something first");
            None
        } else if !state.increment_index() {
            let new_state = ReadyState::create(state);
            new_state.redraw();
            Some(new_state)
        } else {
            state.redraw();
            None
        }
    };

    if let Some(new_state) = new_state {
        *app_state.borrow_mut() = AppState::Ready(new_state)
    }
}

fn handle_touch_start(state: &mut DrawingState, point: Option<Point>) {
    state.set_pressed(true);
    let path = point.map(|x| vec![x]).unwrap_or(vec![]);
    state.add_path(path);
}

fn handle_touch_move(state: &mut DrawingState, point: Point) {
    if state.is_pressed() {
        state.add_point(point);
        state.redraw()
    }
}

fn handle_touch_end(
    app_state: &Rc<RefCell<AppState>>,
    point: Option<Point>,
) -> Result<(), JsValue> {
    match app_state.borrow_mut().deref_mut() {
        AppState::Initial(_) => panic!(),
        AppState::Drawing(state) => {
            if state.is_pressed() {
                state.set_pressed(false);
                if let Some(point) = point {
                    state.add_point(point);
                    state.redraw()
                }
            }
        }
        AppState::Ready(_) => panic!(),
        AppState::Saved(_) => panic!(),
    }

    Ok(())
}

fn handle_advance_btn_click(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    enum Action {
        TurnIntoDrawingState(DrawingState),
        HandleNext,
        TurnIntoSavedState(SavedState),
    }

    let action = {
        match app_state.borrow().deref() {
            AppState::Initial(state) => {
                if state.get_student().is_empty() {
                    alert("Please type your name");
                    None
                } else {
                    let new_state = DrawingState::create(state);
                    new_state.redraw();
                    new_state.subscribe_canvas_events(&app_state)?;
                    new_state.subscribe_to_undo_btn(&app_state)?;

                    Some(Action::TurnIntoDrawingState(new_state))
                }
            }
            AppState::Drawing(_) => Some(Action::HandleNext),
            AppState::Ready(state) => {
                let new_state = SavedState::create(state);
                new_state.redraw();
                Some(Action::TurnIntoSavedState(new_state))
            }
            AppState::Saved(_) => panic!(),
        }
    };

    if let Some(action) = action {
        match action {
            Action::TurnIntoDrawingState(new_state) => *app_state.borrow_mut() = AppState::Drawing(new_state),
            Action::HandleNext => handle_next(&app_state),
            Action::TurnIntoSavedState(new_state) => *app_state.borrow_mut() = AppState::Saved(new_state),
        }
    }

    Ok(())
}

fn subscribe_to_advance_btn(
    app_state: &Rc<RefCell<AppState>>,
    html: &HtmlDom,
) -> Result<(), JsValue> {
    let advance_btn = &html.advance_btn;
    let app_state = app_state.clone();
    advance_btn.on_click(move |_event: MouseEvent| handle_advance_btn_click(&app_state).unwrap())
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let html = HtmlDom::create()?;
    let app_state = Rc::new(RefCell::new(AppState::create(html.clone())));

    subscribe_to_advance_btn(&app_state, &html)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
