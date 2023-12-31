mod app_state;
mod draw;
mod geometry;
mod html;
mod html_state;
mod subscribe_html;
mod subscribe_state;

use crate::app_state::{AppState, DrawingState, InitialState, ReadyState, SavedState};
use crate::draw::Draw;
use crate::html::HtmlDom;
use crate::html_state::Save;
use crate::subscribe_state::StateSubscriber;
use commons::utils::SomeExt;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_commons::html::alert;

fn handle_next(app_state: &Rc<RefCell<AppState<HtmlDom>>>) -> Result<(), JsValue> {
    let new_state = {
        let mut app_state = app_state.borrow_mut();
        let state = app_state.drawing_expected_mut().expect("");
        if state.curr_path().is_empty() {
            alert("Draw something first")?;
            None
        } else if !state.increment_index() {
            let new_state = ReadyState::create(state);
            new_state.draw()?;
            new_state.some()
        } else {
            state.get_view().sketch_pad.borrow_mut().reset();
            state.draw()?;
            None
        }
    };

    if let Some(new_state) = new_state {
        *app_state.borrow_mut() = AppState::Ready(new_state)
    }

    Ok(())
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
                    alert("Please type your name")?;
                    None
                } else {
                    let new_state = DrawingState::create(state);
                    new_state.draw()?;
                    new_state.subscribe(app_state.clone())?;

                    Action::TurnIntoDrawingState(new_state).some()
                }
            }
            AppState::Drawing(_) => Action::HandleNext.some(),
            AppState::Ready(state) => {
                let new_state = state.save()?;
                new_state.draw()?;
                Action::TurnIntoSavedState(new_state).some()
            }
            AppState::Saved(_) => panic!(),
        }
    };

    if let Some(action) = action {
        match action {
            Action::TurnIntoDrawingState(new_state) => {
                *app_state.borrow_mut() = AppState::Drawing(new_state)
            }
            Action::HandleNext => handle_next(&app_state)?,
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

    let app_state = Rc::new(RefCell::new(AppState::create(InitialState::create(html))));

    app_state.borrow().subscribe(app_state.clone())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
