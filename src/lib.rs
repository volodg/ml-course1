mod app;
mod geometry;
mod html;

use crate::app::{AppState, DrawingState, ReadyState};
use crate::geometry::{Point, Rect};
use crate::html::{alert, AddListener, HtmlDom, Visibility};
use itertools::Itertools;
use std::cell::RefCell;
use std::f64;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{MouseEvent, TouchEvent};

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

            html.instructions_spn.set_inner_html(
                std::format!("Please draw a {}", state.get_current_label()).as_str(),
            )
        }
        AppState::Ready(state) => {
            let html = state.get_html_dom();
            html.canvas.set_visible(false);
            html.undo_btn.set_visible(false);
        }
    }

    Ok(())
}

fn turn_to_active_state(app_state: &Rc<RefCell<AppState>>, student: String) -> Result<(), JsValue> {
    let turn_into_active = {
        match app_state.borrow().deref() {
            AppState::Initial(state) => {
                let html = state.get_html_dom();
                html.canvas.set_visible(true);
                html.undo_btn.set_visible(true);
                html.student_input.set_display(false);
                html.advance_btn.set_inner_html("NEXT");

                let app_state = app_state.clone();
                html.advance_btn
                    .on_click(move |_event: MouseEvent| next(&app_state).unwrap())?;

                true
            }
            AppState::Drawing(_) => false,
            AppState::Ready(_) => false,
        }
    };

    if turn_into_active {
        subscribe_canvas_events(&app_state)?;
        let html = app_state.borrow().get_html_dom().clone();
        *app_state.borrow_mut() = AppState::Drawing(DrawingState::create(student, html));
        redraw(app_state.borrow().deref())?
    }

    Ok(())
}

fn next(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    enum State {
        Redraw,
        NewState(AppState),
    }

    let new_state = {
        match app_state.borrow_mut().deref_mut() {
            AppState::Initial(_) => None,
            AppState::Drawing(state) => {
                if state.curr_path().is_empty() {
                    alert("Draw something first");
                    None
                } else if !state.increment_index() {
                    Some(State::NewState(AppState::Ready(ReadyState::create(
                        state.student.clone(),
                        state.get_html_dom().clone(),
                    ))))
                } else {
                    Some(State::Redraw)
                }
            }
            AppState::Ready(_) => None,
        }
    };

    if let Some(new_state) = new_state {
        match new_state {
            State::NewState(state) => *app_state.borrow_mut() = state,
            State::Redraw => (),
        }

        redraw(app_state.borrow().deref())?
    }

    Ok(())
}

fn handle_touch_start(app_state: &mut AppState, point: Option<Point>) {
    match app_state {
        AppState::Initial(_) => {
            // TODO panic!()
        }
        AppState::Drawing(state) => {
            state.set_pressed(true);
            let path = point.map(|x| vec![x]).unwrap_or(vec![]);
            state.add_path(path);
        }
        AppState::Ready(_) => {
            // TODO panic!()
        }
    }
}

fn handle_touch_move(app_state: &Rc<RefCell<AppState>>, point: Point) -> Result<(), JsValue> {
    let redraw_it = {
        match app_state.borrow_mut().deref_mut() {
            AppState::Initial(_) => false,
            AppState::Drawing(state) => {
                if state.is_pressed() {
                    state.add_point(point);
                    true
                } else {
                    false
                }
            }
            AppState::Ready(_) => false,
        }
    };

    if redraw_it {
        redraw(app_state.borrow().deref())?;
    }

    Ok(())
}

fn handle_touch_end(
    app_state: &Rc<RefCell<AppState>>,
    point: Option<Point>,
) -> Result<(), JsValue> {
    let redraw_it = {
        match app_state.borrow_mut().deref_mut() {
            AppState::Initial(_) => false,
            AppState::Drawing(state) => {
                if state.is_pressed() {
                    state.set_pressed(false);
                    if let Some(point) = point {
                        state.add_point(point);
                    }
                    true
                } else {
                    false
                }
            }
            AppState::Ready(_) => false,
        }
    };

    if redraw_it {
        redraw(app_state.borrow().deref())?;
    }

    Ok(())
}

fn subscribe_canvas_events(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let canvas_rect: Rect = app_state
        .borrow()
        .get_html_dom()
        .canvas
        .get_bounding_client_rect()
        .into();
    let adjust_location = move |pos: Point| -> Point {
        Point {
            x: pos.x - canvas_rect.x,
            y: pos.y - canvas_rect.y,
        }
    };

    let canvas = app_state.borrow().get_html_dom().canvas.clone();
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

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let app_state = Rc::new(RefCell::new(AppState::create(HtmlDom::create()?)));

    {
        let undo_btn = app_state.borrow().get_html_dom().undo_btn.clone();
        let app_state = app_state.clone();
        undo_btn.on_click(move |_event: MouseEvent| {
            let redraw_it = {
                match app_state.borrow_mut().deref_mut() {
                    AppState::Initial(_) => false,
                    AppState::Drawing(state) => {
                        state.undo();
                        true
                    }
                    AppState::Ready(_) => false,
                }
            };

            if redraw_it {
                redraw(app_state.borrow().deref()).unwrap()
            }
        })?
    }

    {
        let advance_btn = app_state.borrow().get_html_dom().advance_btn.clone();
        let app_state = app_state.clone();
        advance_btn.on_click(move |_event: MouseEvent| {
            let student = {
                match app_state.borrow().deref() {
                    AppState::Initial(state) => {
                        Some(state.get_html_dom().student_input.value().trim().to_owned())
                    }
                    AppState::Drawing(_) => None,
                    AppState::Ready(_) => None,
                }
            };

            if let Some(student) = student {
                if student.is_empty() {
                    alert("Please type your name");
                } else {
                    turn_to_active_state(&app_state, student).unwrap();
                }
            }
        })?
    }

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
