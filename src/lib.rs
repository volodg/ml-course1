mod geometry;
mod html;

use crate::geometry::{Point, Rect};
use crate::html::{alert, AddListener, HtmlDom, Visibility};
use itertools::Itertools;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{MouseEvent, TouchEvent};

const LABELS: [&str; 8] = [
    "car", "fish", "house", "tree", "bicycle", "guitar", "pencil", "clock",
];

struct AppState {
    student: Option<String>,
    label_index: usize,
    html_dom: HtmlDom,
    pressed: bool,
    paths: Vec<Vec<Point>>,
}

impl AppState {
    fn add_point(&mut self, point: Point) {
        let size = self.paths.len();
        self.paths[size - 1].push(point);
    }

    fn undo(&mut self) {
        while let Some(last) = self.paths.last() {
            if last.is_empty() {
                self.paths.pop();
            } else {
                break;
            }
        }
        self.paths.pop();
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn redraw(app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let html = &app_state.borrow().html_dom;
    html.context.clear_rect(
        0.0,
        0.0,
        html.canvas.width().into(),
        html.canvas.height().into(),
    );

    let mut empty = true;

    for path in &app_state.borrow().paths {
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

    let canvas_is_active = app_state.borrow().student.is_some();

    html.canvas.set_visible(canvas_is_active);
    html.undo_btn.set_visible(canvas_is_active);
    html.student_input.set_display(!canvas_is_active);

    Ok(())
}

fn turn_to_active_state(app_state: &Rc<RefCell<AppState>>, student: String) -> Result<(), JsValue> {
    assert!(app_state.borrow().student.is_none());

    app_state.borrow_mut().student = Some(student);

    let html = &app_state.borrow().html_dom;

    let label = LABELS[app_state.borrow().label_index];
    app_state
        .borrow()
        .html_dom
        .instructions_spn
        .set_inner_html(std::format!("Please draw a {label}").as_str());
    html.advance_btn.set_inner_html("NEXT");

    {
        let app_state = app_state.clone();
        html.advance_btn.on_click(move |_event: MouseEvent| {
            next(&app_state.borrow())
        })?;
    }

    redraw(&app_state)
}

fn next(_app_state: &AppState) {
    log("NEXT clicked")
}

fn handle_touch_start(app_state: &mut AppState, point: Option<Point>) {
    app_state.pressed = true;
    let path = point.map(|x| vec![x]).unwrap_or(vec![]);
    app_state.paths.push(path);
}

fn handle_touch_move(app_state: &Rc<RefCell<AppState>>, point: Point) -> Result<(), JsValue> {
    if app_state.borrow().pressed {
        app_state.borrow_mut().add_point(point);
        redraw(app_state)?;
    }
    Ok(())
}

fn handle_touch_end(app_state: &Rc<RefCell<AppState>>, point: Option<Point>) -> Result<(), JsValue> {
    if app_state.borrow().pressed {
        app_state.borrow_mut().pressed = false;
        if let Some(point) = point {
            app_state.borrow_mut().add_point(point);
        }
        redraw(app_state)?;
    }
    Ok(())
}

fn handle_canvas_events(app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let canvas_rect: Rect = app_state
        .borrow()
        .html_dom
        .canvas
        .get_bounding_client_rect()
        .into();
    let adjust_location = move |pos: Point| -> Point {
        Point {
            x: pos.x - canvas_rect.x,
            y: pos.y - canvas_rect.y,
        }
    };

    let canvas = app_state.borrow().html_dom.canvas.clone();
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
        canvas.add_listener("touchend", move |event: TouchEvent| {
            let point = event.try_into().ok().map(adjust_location);
            handle_touch_end(&app_state, point).unwrap()
        })?
    }

    Ok(())
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let app_state = Rc::new(RefCell::new(AppState {
        student: None,
        label_index: 0,
        html_dom: HtmlDom::create()?,
        pressed: false,
        paths: Vec::new(),
    }));

    handle_canvas_events(app_state.clone())?;

    {
        let undo_btn = &app_state.borrow().html_dom.undo_btn;
        let app_state = app_state.clone();
        undo_btn.on_click(move |_event: MouseEvent| {
            app_state.borrow_mut().undo();
            redraw(&app_state).unwrap()
        })?
    }

    {
        let advance_btn = &app_state.borrow().html_dom.advance_btn;
        let app_state = app_state.clone();
        advance_btn.on_click(move |_event: MouseEvent| {
            if app_state.borrow().student.is_some() {
                return;
            }

            let student = app_state
                .borrow()
                .html_dom
                .student_input
                .value()
                .trim()
                .to_owned();
            if student.is_empty() {
                alert("Please type your name");
            } else {
                turn_to_active_state(&app_state, student).unwrap();
            }
        })?
    }

    redraw(&app_state)?;

    Ok(())
}
