use crate::geometry::Point;
use crate::html::HtmlDom;
use crate::html::Visibility;
use itertools::Itertools;
use web_sys::HtmlCanvasElement;

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
    pub fn get_html_dom(&self) -> &HtmlDom {
        &self.html_dom
    }

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
    pub fn create(student: String, html_dom: HtmlDom) -> Self {
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

    pub fn get_html_dom(&self) -> &HtmlDom {
        &self.html_dom
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
    pub fn create(html_dom: HtmlDom) -> Self {
        Self { html_dom }
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
    pub fn get_html_dom(&self) -> &HtmlDom {
        match self {
            Self::Initial(state) => &state.html_dom,
            Self::Drawing(state) => &state.html_dom,
            Self::Ready(state) => &state.html_dom,
            Self::Saved(state) => &state.html_dom,
        }
    }

    pub fn get_canvas(&self) -> &HtmlCanvasElement {
        &self.get_html_dom().canvas
    }
}

impl AppState {
    pub fn create(html_dom: HtmlDom) -> Self {
        Self::Initial(InitialState { html_dom })
    }
}
