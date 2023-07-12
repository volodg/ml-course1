mod app;
mod canvas;
mod geometry;
mod html;

use crate::app::{AppState, DrawingState, ReadyState, SavedState};
use crate::canvas::subscribe_canvas_events;
use crate::geometry::Point;
use crate::html::{alert, AddListener, HtmlDom, Visibility};
use itertools::Itertools;
use std::cell::RefCell;
use std::f64;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn redraw(app_state: &AppState) -> Result<(), JsValue> {
    match app_state {
        AppState::Initial(_) => (),
        AppState::Drawing(state) => {
            let html = state.get_html_dom();
            html.context.clear_rect(
                0.0,
                0.0,
                html.canvas.width().into(),
                html.canvas.height().into(),
            );

            let mut empty = true;

            for path in state.curr_path() {
                if path.is_empty() {
                    continue;
                }
                empty = false;

                for (from, to) in path.iter().tuple_windows() {
                    html.context.begin_path();
                    html.context.set_line_width(3.0);
                    html.context.set_line_cap("round");
                    html.context.set_line_join("round");

                    html.context.move_to(from.x as f64, from.y as f64);
                    html.context.line_to(to.x as f64, to.y as f64);

                    html.context.stroke();
                }
            }

            html.undo_btn.set_disabled(empty);
        }
        AppState::Ready(state) => {
            let html = state.get_html_dom();
            html.canvas.set_visible(false); // TODO
            html.undo_btn.set_visible(false);

            html.instructions_spn.set_inner_html("Thank you!");
            html.advance_btn.set_inner_html("SAVE");
        }
        AppState::Saved(_) => (),
    }

    Ok(())
}

fn turn_into_saved_state(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let html = app_state.borrow().get_html_dom().clone();

    html.advance_btn.set_display(false);
    html.instructions_spn.set_inner_html(
        "Take you downloaded file and place it along side the others in the dataset!",
    );

    *app_state.borrow_mut() = AppState::Saved(SavedState::create(html));

    Ok(())
}

fn turn_into_drawing_state(
    app_state: &Rc<RefCell<AppState>>,
    student: String,
) -> Result<(), JsValue> {
    let html = app_state.borrow().get_html_dom().clone();

    html.canvas.set_visible(true);
    html.undo_btn.set_visible(true);
    html.student_input.set_display(false);
    html.advance_btn.set_inner_html("NEXT");

    subscribe_canvas_events(&app_state)?;
    subscribe_to_undo_btn(&app_state)?;

    let new_state = DrawingState::create(student, html);
    let label = new_state.get_current_label().to_owned();
    app_state.borrow().get_html_dom().draw_a_task_label(label);

    *app_state.borrow_mut() = AppState::Drawing(new_state);

    redraw(app_state.borrow().deref())?;

    Ok(())
}

fn handle_next(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    enum Action {
        Redraw(String),
        NewState(AppState),
    }

    let new_state = {
        match app_state.borrow_mut().deref_mut() {
            AppState::Initial(_) => panic!(),
            AppState::Drawing(state) => {
                if state.curr_path().is_empty() {
                    alert("Draw something first");
                    None
                } else if !state.increment_index() {
                    Some(Action::NewState(AppState::Ready(ReadyState::create(
                        state.student.clone(),
                        state.get_html_dom().clone(),
                    ))))
                } else {
                    Some(Action::Redraw(state.get_current_label().to_owned()))
                }
            }
            AppState::Ready(_) => None,
            AppState::Saved(_) => panic!(),
        }
    };

    if let Some(new_state) = new_state {
        match new_state {
            Action::NewState(state) => *app_state.borrow_mut() = state,
            Action::Redraw(label) => app_state.borrow().get_html_dom().draw_a_task_label(label),
        };

        redraw(app_state.borrow().deref())?
    }

    Ok(())
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
    {
        match app_state.borrow_mut().deref_mut() {
            AppState::Initial(_) => panic!(),
            AppState::Drawing(state) => {
                if state.is_pressed() {
                    state.add_point(point);
                    true
                } else {
                    false
                }
            }
            AppState::Ready(_) => panic!(),
            AppState::Saved(_) => panic!(),
        }
    };

    redraw(app_state.borrow().deref())?;

    Ok(())
}

fn handle_touch_end(
    app_state: &Rc<RefCell<AppState>>,
    point: Option<Point>,
) -> Result<(), JsValue> {
    {
        match app_state.borrow_mut().deref_mut() {
            AppState::Initial(_) => panic!(),
            AppState::Drawing(state) => {
                if state.is_pressed() {
                    state.set_pressed(false);
                    if let Some(point) = point {
                        state.add_point(point);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            AppState::Ready(_) => panic!(),
            AppState::Saved(_) => panic!(),
        }
    };

    redraw(app_state.borrow().deref())?;

    Ok(())
}

fn subscribe_to_undo_btn(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let undo_btn = app_state.borrow().get_html_dom().undo_btn.clone();
    let app_state = app_state.clone();
    undo_btn.on_click(move |_event: MouseEvent| {
        {
            match app_state.borrow_mut().deref_mut() {
                AppState::Initial(_) => panic!(),
                AppState::Drawing(state) => {
                    state.undo();
                    true
                }
                AppState::Ready(_) => panic!(),
                AppState::Saved(_) => panic!(),
            }
        };

        redraw(app_state.borrow().deref()).unwrap()
    })
}

fn handle_advance_btn_click(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    enum Action {
        Register(String),
        Next,
        Save,
    }

    let action = {
        match app_state.borrow().deref() {
            AppState::Initial(state) => Some(Action::Register(
                state.get_html_dom().student_input.value().trim().to_owned(),
            )),
            AppState::Drawing(_) => Some(Action::Next),
            AppState::Ready(_) => Some(Action::Save),
            AppState::Saved(_) => panic!(),
        }
    };

    if let Some(action) = action {
        match action {
            Action::Register(student) => {
                if student.is_empty() {
                    alert("Please type your name")
                } else {
                    turn_into_drawing_state(&app_state, student)?
                }
            }
            Action::Next => handle_next(&app_state)?,
            Action::Save => turn_into_saved_state(&app_state)?,
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

    redraw(app_state.borrow().deref())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        assert!(true);
    }
}
