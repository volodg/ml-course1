use crate::canvas::StateSubscriber;
use crate::geometry::Point;
use crate::html::AddListener;
use crate::html::HtmlDom;
use crate::html::Visibility;
use itertools::Itertools;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::MouseEvent;

struct Drawing {
    label: &'static str,
    pressed: bool,
    paths: Vec<Vec<Point>>,
}

impl Drawing {
    fn create(label: &'static str) -> Self {
        Self {
            label,
            pressed: false,
            paths: vec![],
        }
    }
}

pub struct InitialState {
    html_dom: HtmlDom,
}

impl InitialState {
    pub fn get_student(&self) -> String {
        self.html_dom.student_input.value().trim().to_owned()
    }
}

pub struct DrawingState {
    pub student: String,
    label_index: usize,
    html_dom: HtmlDom,
    drawings: [Drawing; 8],
}

impl DrawingState {
    pub fn create(state: &InitialState) -> Self {
        let student = state.get_student();
        let html_dom = state.html_dom.clone();
        Self {
            student,
            label_index: 0,
            html_dom,
            drawings: [
                Drawing::create("car"),
                Drawing::create("fish"),
                Drawing::create("house"),
                Drawing::create("tree"),
                Drawing::create("bicycle"),
                Drawing::create("guitar"),
                Drawing::create("pencil"),
                Drawing::create("clock"),
            ],
        }
    }

    fn redraw_a_task_label(&self) {
        let label = self.get_current_label();
        self.html_dom
            .instructions_spn
            .set_inner_html(std::format!("Please draw a {label}").as_str());
    }

    pub fn subscribe_canvas_events(
        &self,
        app_state: &Rc<RefCell<AppState>>,
    ) -> Result<(), JsValue> {
        self.html_dom.canvas.subscribe(app_state)
    }

    pub fn subscribe_to_undo_btn(&self, app_state: &Rc<RefCell<AppState>>) -> Result<(), JsValue> {
        let undo_btn = self.html_dom.undo_btn.clone();
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

    pub fn redraw(&self) {
        self.html_dom.canvas.set_visible(true);
        self.html_dom.undo_btn.set_visible(true);
        self.html_dom.student_input.set_display(false);
        self.html_dom.advance_btn.set_inner_html("NEXT");

        self.html_dom.context.clear_rect(
            0.0,
            0.0,
            self.html_dom.canvas.width().into(),
            self.html_dom.canvas.height().into(),
        );

        let mut empty = true;

        for path in self.curr_path() {
            if path.is_empty() {
                continue;
            }
            empty = false;

            for (from, to) in path.iter().tuple_windows() {
                self.html_dom.context.begin_path();
                self.html_dom.context.set_line_width(3.0);
                self.html_dom.context.set_line_cap("round");
                self.html_dom.context.set_line_join("round");

                self.html_dom.context.move_to(from.x as f64, from.y as f64);
                self.html_dom.context.line_to(to.x as f64, to.y as f64);

                self.html_dom.context.stroke();
            }
        }

        self.html_dom.undo_btn.set_disabled(empty);
        self.redraw_a_task_label()
    }

    pub fn set_pressed(&mut self, value: bool) {
        self.drawings[self.label_index].pressed = value
    }

    pub fn is_pressed(&self) -> bool {
        self.drawings[self.label_index].pressed
    }

    pub fn curr_path(&self) -> &Vec<Vec<Point>> {
        &self.drawings[self.label_index].paths
    }

    fn curr_path_mut(&mut self) -> &mut Vec<Vec<Point>> {
        &mut self.drawings[self.label_index].paths
    }

    pub fn add_point(&mut self, point: Point) {
        let size = self.curr_path().len();
        self.curr_path_mut()[size - 1].push(point);
    }

    pub fn add_path(&mut self, points: Vec<Point>) {
        self.curr_path_mut().push(points);
    }

    pub fn undo(&mut self) {
        while let Some(last) = self.curr_path().last() {
            if last.is_empty() {
                self.curr_path_mut().pop();
            } else {
                break;
            }
        }
        self.curr_path_mut().pop();
    }

    fn get_current_label(&self) -> &str {
        self.drawings[self.label_index].label
    }

    pub fn increment_index(&mut self) -> bool {
        self.label_index += 1;
        self.label_index < self.drawings.len()
    }
}

pub struct ReadyState {
    #[allow(dead_code)]
    student: String,
    html_dom: HtmlDom,
}

impl ReadyState {
    pub fn create(state: &DrawingState) -> Self {
        let student = state.student.clone();
        let html_dom = state.html_dom.clone();
        Self { student, html_dom }
    }

    pub fn redraw(&self) {
        self.html_dom.canvas.set_visible(false);
        self.html_dom.undo_btn.set_visible(false);

        self.html_dom.instructions_spn.set_inner_html("Thank you!");
        self.html_dom.advance_btn.set_inner_html("SAVE");
    }
}

pub struct SavedState {
    html_dom: HtmlDom,
}

impl SavedState {
    pub fn create(state: &ReadyState) -> Self {
        Self {
            html_dom: state.html_dom.clone(),
        }
    }

    pub fn redraw(&self) {
        self.html_dom.advance_btn.set_display(false);
        self.html_dom.instructions_spn.set_inner_html(
            "Take you downloaded file and place it along side the others in the dataset!",
        );
    }
}

pub enum AppState {
    Initial(InitialState),
    Drawing(DrawingState),
    Ready(ReadyState),
    Saved(SavedState),
}

impl AppState {
    pub fn create(html_dom: HtmlDom) -> Self {
        Self::Initial(InitialState { html_dom })
    }

    pub fn drawing_expected_mut(&mut self) -> &mut DrawingState {
        match self {
            AppState::Initial(_) => panic!("unexpected state"),
            AppState::Drawing(state) => state,
            AppState::Ready(_) => panic!("unexpected state"),
            AppState::Saved(_) => panic!("unexpected state"),
        }
    }
}
