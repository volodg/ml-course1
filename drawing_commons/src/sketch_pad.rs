use crate::models::DrawingPaths;
use commons::math::Point;
use commons::utils::OkExt;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_commons::html::AddListener;
use web_commons::html::Visibility;
use web_sys::{
    window, CanvasRenderingContext2d, Document, Element, HtmlButtonElement, HtmlCanvasElement,
    HtmlElement, MouseEvent, TouchEvent,
};

pub struct SketchPad {
    document: Document,
    container: HtmlElement,
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    undo_btn: HtmlButtonElement,
    on_update: Option<Rc<RefCell<dyn FnMut(&Vec<Vec<Point>>)>>>,
    paths: Vec<Vec<Point>>,
    is_drawing: bool,
}

impl SketchPad {
    pub fn create(container_id: &str) -> Result<Rc<RefCell<Self>>, JsValue> {
        let document = window().expect("").document().expect("");
        let container = document
            .get_element_by_id(container_id)
            .unwrap()
            .dyn_into::<HtmlElement>()?;

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;
        let size = 400;
        canvas.set_width(size);
        canvas.set_height(size);
        canvas.style().set_property("background-color", "black")?;
        canvas
            .style()
            .set_property("box-shadow", "0px 0px 10px 2px white")?;

        container.append_child(&canvas)?;

        let line_break = document.create_element("br")?.dyn_into::<Element>()?;
        container.append_child(&line_break)?;

        let undo_btn = document
            .create_element("button")?
            .dyn_into::<HtmlButtonElement>()?;
        undo_btn.set_inner_html("UNDO");
        container.append_child(&undo_btn)?;

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let result = Self {
            document,
            container,
            canvas,
            context,
            undo_btn,
            on_update: None,
            paths: vec![],
            is_drawing: false,
        };

        result.draw();

        let result = Rc::new(RefCell::new(result));

        Self::add_event_listeners(&result)?;

        result.ok()
    }

    pub fn add_shadow(&self) {
        let mut canvas_css = self.canvas.style().css_text();
        canvas_css.push_str("outline:10000px solid rgba(0,0,0,0.7)");
        self.canvas.style().set_css_text(&canvas_css)
    }

    pub fn set_visible(&self, visible: bool) -> Result<(), JsValue> {
        self.container.set_visible(visible)
    }

    pub fn set_on_update(&mut self, on_update: Rc<RefCell<dyn FnMut(&DrawingPaths<Point>)>>) {
        self.on_update = Some(on_update)
    }

    fn get_mouse(&self, event: MouseEvent) -> Point {
        let rect = self.canvas.get_bounding_client_rect();
        Point {
            x: event.client_x() as f64 - rect.left(),
            y: event.client_y() as f64 - rect.top(),
        }
    }

    fn handle_touch_start(&mut self, point: Point) {
        self.paths.push(vec![point]);
        self.is_drawing = true;
    }

    fn handle_touch_move(&mut self, point: Point) {
        if self.is_drawing {
            let last_index = self.paths.len() - 1;
            self.paths[last_index].push(point);
            self.draw();
        }
    }

    fn handle_touch_end(&mut self) {
        if self.is_drawing {
            self.is_drawing = false;
            self.trigger_update();
        }
    }

    fn add_event_listeners(sketch_pad: &Rc<RefCell<Self>>) -> Result<(), JsValue> {
        let sketch_pad_copy = sketch_pad.clone();
        sketch_pad
            .borrow()
            .canvas
            .add_listener("pointerdown", move |event: MouseEvent| {
                let mut sketch_pad = sketch_pad_copy.borrow_mut();
                event.prevent_default();
                let mouse = sketch_pad.get_mouse(event);
                sketch_pad.handle_touch_start(mouse);
            })?;

        let sketch_pad_copy = sketch_pad.clone();
        sketch_pad
            .borrow()
            .canvas
            .add_listener("pointermove", move |event: MouseEvent| {
                let mut sketch_pad = sketch_pad_copy.borrow_mut();
                event.prevent_default();
                let mouse = sketch_pad.get_mouse(event);
                sketch_pad.handle_touch_move(mouse);
            })?;

        let sketch_pad_copy = sketch_pad.clone();
        sketch_pad
            .borrow()
            .document
            .add_listener("pointerup", move |_event: MouseEvent| {
                sketch_pad_copy.borrow_mut().handle_touch_end();
            })?;

        let sketch_pad_copy = sketch_pad.clone();
        sketch_pad
            .borrow()
            .canvas
            .add_listener("touchend", move |_event: TouchEvent| {
                sketch_pad_copy.borrow_mut().handle_touch_end();
            })?;

        let sketch_pad_copy = sketch_pad.clone();
        sketch_pad
            .borrow()
            .undo_btn
            .on_click(move |_event: MouseEvent| {
                let mut sketch_pad = sketch_pad_copy.borrow_mut();
                sketch_pad.paths.pop();
                sketch_pad.draw();
            })?;

        Ok(())
    }

    pub fn trigger_update(&self) {
        if let Some(on_update) = self.on_update.clone() {
            on_update.borrow_mut()(&self.paths);
        }
    }

    fn draw_path(&self) {
        for path in &self.paths {
            if path.is_empty() {
                continue;
            }

            for (from, to) in path.iter().tuple_windows() {
                self.context.begin_path();
                self.context.set_line_width(3.0);
                self.context.set_stroke_style(&JsValue::from_str("white"));
                self.context.set_line_cap("round");
                self.context.set_line_join("round");

                self.context.move_to(from.x as f64, from.y as f64);
                self.context.line_to(to.x as f64, to.y as f64);

                self.context.stroke();
            }
        }
    }

    pub fn reset(&mut self) {
        self.paths.clear();
        self.draw();
    }

    fn draw(&self) {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width().into(),
            self.canvas.height().into(),
        );

        self.draw_path();

        self.undo_btn.set_disabled(self.paths.is_empty());
    }
}
