use itertools::Itertools;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{MouseEvent, TouchEvent};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct AppState {
    context: Rc<web_sys::CanvasRenderingContext2d>,
    pressed: bool,
    paths: Vec<Vec<Point>>,
}

impl AppState {
    fn add_point(&mut self, point: Point) {
        let size = self.paths.len();
        self.paths[size - 1].push(point);
    }
}

impl From<MouseEvent> for Point {
    fn from(event: MouseEvent) -> Self {
        Self {
            x: event.offset_x(),
            y: event.offset_y(),
        }
    }
}

impl TryFrom<TouchEvent> for Point {
    type Error = ();

    fn try_from(event: TouchEvent) -> Result<Self, Self::Error> {
        match event.touches().get(0) {
            Some(touch) => Ok(Self {
                x: touch.screen_x(),
                y: touch.screen_y(),
            }),
            None => Err(())
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn redraw(state: &AppState) {
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

fn handle_touch_start(app_state: &mut AppState, point: Point) {
    app_state.pressed = true;
    app_state.paths.push(vec![point]);
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

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = Rc::new(context);

    let app_state = Rc::new(RefCell::new(AppState {
        context,
        pressed: false,
        paths: Vec::new(),
    }));

    {
        let app_state = app_state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            handle_touch_start(&mut app_state.borrow_mut(), event.into())
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
            if let Some(point) = event.try_into().ok() {
                handle_touch_start(&mut app_state.borrow_mut(), point)
            }
        });
        canvas.add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let app_state = app_state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
            if let Some(point) = event.try_into().ok() {
                handle_touch_move(&mut app_state.borrow_mut(), point)
            }
        });
        canvas.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
            handle_touch_end(&mut app_state.borrow_mut(), event.try_into().ok())
        });
        canvas.add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
