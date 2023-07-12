mod geometry;

use itertools::Itertools;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{MouseEvent, TouchEvent};
use crate::geometry::{Point, Rect};

struct AppState {
    context: Rc<web_sys::CanvasRenderingContext2d>,
    canvas: Rc<web_sys::HtmlCanvasElement>,
    pressed: bool,
    paths: Vec<Vec<Point>>,
}

impl AppState {
    fn add_point(&mut self, point: Point) {
        let size = self.paths.len();
        self.paths[size - 1].push(point);
    }

    fn undo(&mut self) {
        self.paths.pop();
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn redraw(state: &AppState) {
    state.context.clear_rect(0.0, 0.0, state.canvas.width().into(), state.canvas.height().into());

    for path in &state.paths {
        if path.is_empty() {
            continue;
        }

        for (from, to) in path.iter().tuple_windows() {
            state.context.begin_path();
            state.context.set_line_width(3.0);
            state.context.set_line_cap("round");
            state.context.set_line_join("round");

            state.context.move_to(from.x as f64, from.y as f64);
            state.context.line_to(to.x as f64, to.y as f64);

            state.context.stroke();
        }
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

fn handle_canvas_events(app_state: Rc<RefCell<AppState>>) -> Result<(), JsValue> {
    let canvas_rect: Rect = app_state.borrow().canvas.get_bounding_client_rect().into();
    let adjust_location = move |pos: Point| -> Point {
        Point {
            x: pos.x - canvas_rect.x,
            y: pos.y - canvas_rect.y,
        }
    };

    let canvas = app_state.borrow().canvas.clone();
    {
        let app_state = app_state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            handle_touch_start(&mut app_state.borrow_mut(), Some(event.into()))
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let app_state = app_state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            handle_touch_move(&mut app_state.borrow_mut(), event.into())
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let app_state = app_state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            handle_touch_end(&mut app_state.borrow_mut(), Some(event.into()))
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let app_state = app_state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
            let point = event.try_into().ok().map(adjust_location);
            handle_touch_start(&mut app_state.borrow_mut(), point)
        });
        canvas.add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let app_state = app_state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
            let point = event.try_into().ok().map(adjust_location);
            if let Some(point) = point {
                handle_touch_move(&mut app_state.borrow_mut(), point)
            }
        });
        canvas.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
        closure.forget();
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
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let app_state = Rc::new(RefCell::new(AppState {
        context: Rc::new(context),
        canvas: Rc::new(canvas),
        pressed: false,
        paths: Vec::new(),
    }));

    handle_canvas_events(app_state.clone())?;

    let undo_btn = document.get_element_by_id("undo").unwrap();
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
            app_state.borrow_mut().undo();
            redraw(&app_state.borrow())
        });
        undo_btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
