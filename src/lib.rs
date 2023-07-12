mod app;
mod canvas;
mod geometry;
mod html;

use crate::app::{AppState, DrawingState, ReadyState, SavedState};
use crate::canvas::subscribe_canvas_events;
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

fn turn_into_saved_state(
    app_state: &Rc<RefCell<AppState>>,
    html_dom: HtmlDom,
) {
    let new_state = SavedState::create(html_dom);
    new_state.redraw();

    *app_state.borrow_mut() = AppState::Saved(new_state)
}

fn turn_into_drawing_state(
    app_state: &Rc<RefCell<AppState>>,
    student: String,
    html_dom: HtmlDom
) -> Result<(), JsValue> {
    subscribe_canvas_events(&app_state)?;
    subscribe_to_undo_btn(&app_state)?;

    let new_state = DrawingState::create(student, html_dom);
    new_state.redraw();

    *app_state.borrow_mut() = AppState::Drawing(new_state);

    Ok(())
}

fn handle_next(app_state: &Rc<RefCell<AppState>>) {
    enum Action {
        IntoReady(ReadyState),
    }

    let new_state = {
        match app_state.borrow_mut().deref_mut() {
            AppState::Initial(_) => panic!(),
            AppState::Drawing(state) => {
                if state.curr_path().is_empty() {
                    alert("Draw something first");
                    None
                } else if !state.increment_index() {
                    let new_state = ReadyState::create(state);
                    new_state.redraw();
                    Some(Action::IntoReady(new_state))
                } else {
                    state.redraw();
                    None
                }
            }
            AppState::Ready(_) => None,
            AppState::Saved(_) => panic!(),
        }
    };

    if let Some(new_state) = new_state {
        match new_state {
            Action::IntoReady(state) => *app_state.borrow_mut() = AppState::Ready(state),
        };
    }
}

fn handle_touch_start(app_state: &mut AppState, point: Option<Point>) {
    match app_state {
        AppState::Initial(_) => panic!(),
        AppState::Drawing(state) => {
            state.set_pressed(true);
            let path = point.map(|x| vec![x]).unwrap_or(vec![]);
            state.add_path(path);
        }
        AppState::Ready(_) => panic!(),
        AppState::Saved(_) => panic!(),
    }
}

fn handle_touch_move(app_state: &Rc<RefCell<AppState>>, point: Point) -> Result<(), JsValue> {
    match app_state.borrow_mut().deref_mut() {
        AppState::Initial(_) => panic!(),
        AppState::Drawing(state) => {
            if state.is_pressed() {
                state.add_point(point);
                state.redraw()
            }
        }
        AppState::Ready(_) => panic!(),
        AppState::Saved(_) => panic!(),
    }

    Ok(())
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

fn subscribe_to_undo_btn(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let undo_btn = app_state.borrow().get_html_dom().undo_btn.clone();
    let app_state = app_state.clone();
    undo_btn.on_click(
        move |_event: MouseEvent| match app_state.borrow_mut().deref_mut() {
            AppState::Initial(_) => panic!(),
            AppState::Drawing(state) => {
                state.undo();
                state.redraw();
            }
            AppState::Ready(_) => panic!(),
            AppState::Saved(_) => panic!(),
        },
    )
}

fn handle_advance_btn_click(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    enum Action {
        Register(String, HtmlDom),
        Next,
        Save(HtmlDom),
    }

    let action = {
        match app_state.borrow().deref() {
            AppState::Initial(state) => Some(Action::Register(
                state.get_student(), state.get_html_dom().clone()
            )),
            AppState::Drawing(_) => Some(Action::Next),
            AppState::Ready(state) => Some(Action::Save(state.get_html_dom().clone())),
            AppState::Saved(_) => panic!(),
        }
    };

    if let Some(action) = action {
        match action {
            Action::Register(student, html) => {
                if student.is_empty() {
                    alert("Please type your name")
                } else {
                    turn_into_drawing_state(&app_state, student, html)?
                }
            }
            Action::Next => handle_next(&app_state),
            Action::Save(html) => turn_into_saved_state(&app_state, html),
        }
    }

    Ok(())
}

fn subscribe_to_advance_btn(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let advance_btn = app_state.borrow().get_html_dom().advance_btn.clone();
    let app_state = app_state.clone();
    advance_btn.on_click(move |_event: MouseEvent| handle_advance_btn_click(&app_state).unwrap())
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let app_state = Rc::new(RefCell::new(AppState::create(HtmlDom::create()?)));

    subscribe_to_advance_btn(&app_state)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
