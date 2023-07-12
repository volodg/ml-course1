mod geometry;
mod html;

use crate::geometry::{Point, Rect};
use crate::html::{alert, HtmlDom, Visibility};
use itertools::Itertools;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{MouseEvent, TouchEvent, HtmlElement};

const LABELS: [&str; 8] = ["car", "fish", "house", "tree", "bicycle", "guitar", "pencil", "clock"];

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

fn redraw(state: &AppState) {
    let html = &state.html_dom;
    html.context.clear_rect(
        0.0,
        0.0,
        html.canvas.width().into(),
        html.canvas.height().into(),
    );

    let mut empty = true;

    for path in &state.paths {
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

    let canvas_is_active = state.student.is_some();

    html.canvas.set_visible(canvas_is_active);
    html.undo_btn.set_visible(canvas_is_active);
    html.student_input.set_display(!canvas_is_active);

    if canvas_is_active {
        let label = LABELS[state.label_index];
        state.html_dom.instructions_spn.set_inner_html(std::format!("Please draw a {label}").as_str());
        html.advance_btn.set_inner_html("NEXT");
    }
}

fn handle_touch_start(app_state: &mut AppState, point: Option<Point>) {
    app_state.pressed = true;
    let path = point.map(|x| vec![x]).unwrap_or(vec![]);
    app_state.paths.push(path);
}

fn handle_touch_move(app_state: &mut AppState, point: Point) {
    if app_state.pressed {
        app_state.add_point(point);
        redraw(app_state);
    }
}

fn handle_touch_end(app_state: &mut AppState, point: Option<Point>) {
    if app_state.pressed {
        app_state.pressed = false;
        if let Some(point) = point {
            app_state.add_point(point);
        }
        redraw(app_state);
    }
}

trait AddListener {
    fn add_listener<Event: wasm_bindgen::convert::FromWasmAbi + 'static, F>(&self, name: &str, listener: F) -> Result<(), JsValue> where F: FnMut(Event) + 'static;
}

impl AddListener for HtmlElement {
    fn add_listener<Event: wasm_bindgen::convert::FromWasmAbi + 'static, F>(&self, name: &str, mut listener: F) -> Result<(), JsValue> where F: FnMut(Event) + 'static {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| {
            listener(event)
        });
        self.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())?;
        closure.forget();
        Ok(())
    }
}

fn handle_canvas_events(app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let canvas_rect: Rect = app_state.borrow().html_dom.canvas.get_bounding_client_rect().into();
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
            handle_touch_move(&mut app_state.borrow_mut(), event.into())
        })?
    }
    {
        let app_state = app_state.clone();
        canvas.add_listener("mouseup", move |event: MouseEvent| {
            handle_touch_end(&mut app_state.borrow_mut(), Some(event.into()))
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
                handle_touch_move(&mut app_state.borrow_mut(), point)
            }
        })?
    }
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
            let point = event.try_into().ok().map(adjust_location);
            handle_touch_end(&mut app_state.borrow_mut(), point)
        });
        canvas.add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
        closure.forget();
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
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
            app_state.borrow_mut().undo();
            redraw(&app_state.borrow())
        });
        undo_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let advance_btn = &app_state.borrow().html_dom.advance_btn;
        let app_state = app_state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
            let student = app_state.borrow().html_dom.student_input.value().trim().to_owned();
            if student == "" {
                alert("Please type your name");
            } else {
                app_state.borrow_mut().student = Some(student);
                redraw(&app_state.borrow());
            }
        });
        advance_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    redraw(&app_state.borrow());

    Ok(())
}
