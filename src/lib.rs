mod app_state;
mod draw;
mod geometry;
mod html;
mod subscribe_html;
mod subscribe_state;

use crate::app_state::{AppState, DrawingState, InitialState, ReadyState, SavedState};
use crate::draw::Draw;
use crate::geometry::Point;
use crate::html::{alert, HtmlDom};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::subscribe_state::StateSubscriber;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn handle_next(app_state: &Rc<RefCell<AppState<HtmlDom>>>) {
    let new_state = {
        let mut app_state = app_state.borrow_mut();
        let state = app_state.drawing_expected_mut();
        if state.curr_path().is_empty() {
            alert("Draw something first");
            None
        } else if !state.increment_index() {
            let new_state = ReadyState::create(state);
            new_state.draw();
            Some(new_state)
        } else {
            state.draw();
            None
        }
    };

    if let Some(new_state) = new_state {
        *app_state.borrow_mut() = AppState::Ready(new_state)
    }
}

fn handle_touch_start(state: &mut DrawingState<HtmlDom>, point: Option<Point>) {
    state.set_pressed(true);
    let path = point.map(|x| vec![x]).unwrap_or(vec![]);
    state.add_path(path);
}

fn handle_touch_move(state: &mut DrawingState<HtmlDom>, point: Point) {
    if state.is_pressed() {
        state.add_point(point);
        state.draw()
    }
}

fn handle_touch_end(state: &mut DrawingState<HtmlDom>, point: Option<Point>) {
    if state.is_pressed() {
        state.set_pressed(false);
        if let Some(point) = point {
            state.add_point(point);
            state.draw()
        }
    }
}

fn handle_advance_btn_click(app_state: &Rc<RefCell<AppState<HtmlDom>>>) -> Result<(), JsValue> {
    enum Action {
        TurnIntoDrawingState(DrawingState<HtmlDom>),
        HandleNext,
        TurnIntoSavedState(SavedState<HtmlDom>),
    }

    let action = {
        match app_state.borrow().deref() {
            AppState::Initial(state) => {
                if state.get_student().is_empty() {
                    alert("Please type your name");
                    None
                } else {
                    let new_state = DrawingState::create(state);
                    new_state.draw();
                    new_state.subscribe(&app_state)?;

                    Some(Action::TurnIntoDrawingState(new_state))
                }
            }
            AppState::Drawing(_) => Some(Action::HandleNext),
            AppState::Ready(state) => {
                let new_state = SavedState::create(state);
                new_state.draw();
                Some(Action::TurnIntoSavedState(new_state))
            }
            AppState::Saved(_) => panic!(),
        }
    };

    if let Some(action) = action {
        match action {
            Action::TurnIntoDrawingState(new_state) => {
                *app_state.borrow_mut() = AppState::Drawing(new_state)
            }
            Action::HandleNext => handle_next(&app_state),
            Action::TurnIntoSavedState(new_state) => {
                *app_state.borrow_mut() = AppState::Saved(new_state)
            }
        }
    }

    Ok(())
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let html = HtmlDom::create()?;

    let app_state = InitialState::create(html);
    let app_state = AppState::create(app_state);
    let app_state = Rc::new(RefCell::new(app_state));

    app_state.borrow().subscribe(&app_state)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
