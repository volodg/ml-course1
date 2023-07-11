use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use itertools::Itertools;
use wasm_bindgen::prelude::*;
use web_sys::MouseEvent;

struct Point {
    x: i32,
    y: i32,
}

struct AppState {
    context: Rc<web_sys::CanvasRenderingContext2d>,
    pressed: bool,
    paths: Vec<Vec<Point>>
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

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// fn handle_redraw(canvas: &web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
//     log("setup handle_on_draw");
//
//     let closure = Closure::<dyn FnMut(_)>::new(move |event: Event| {
//         // Код обработчика события
//         log("redraw event triggered");
//     });
//
//     canvas.add_event_listener_with_callback("redraw", closure.as_ref().unchecked_ref())?;
//     closure.forget();
//
//     Ok(())
// }

fn draw(state: &AppState) {
    for path in &state.paths {
        if path.is_empty() {
            continue
        }

        for (from, to) in path.iter().tuples() {
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

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    //
    // handle_redraw(&canvas)?;

    let context = Rc::new(context);

    let app_state = Rc::new(RefCell::new(AppState {
        context,
        pressed: false,
        paths: Vec::new(),
    }));

    {
        let app_state = app_state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
            app_state.borrow_mut().pressed = true;
            app_state.borrow_mut().paths.push(Vec::new());
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let app_state = app_state.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            if app_state.borrow().pressed {
                app_state.borrow_mut().add_point(event.into());
                draw(&app_state.borrow());
            }
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            app_state.borrow_mut().pressed = false;
            app_state.borrow_mut().add_point(event.into());
            draw(&app_state.borrow());
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}