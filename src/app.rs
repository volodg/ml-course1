use crate::geometry::Point;
use crate::html::HtmlDom;

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

    pub fn get_html_dom(&self) -> &HtmlDom {
        &self.html_dom
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

    pub fn get_current_label(&self) -> &str {
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
    pub fn create(student: String, html_dom: HtmlDom) -> Self {
        Self { student, html_dom }
    }

    pub fn get_html_dom(&self) -> &HtmlDom {
        &self.html_dom
    }
}

pub enum AppState {
    Initial(InitialState),
    Drawing(DrawingState),
    Ready(ReadyState),
}

impl AppState {
    pub fn get_html_dom(&self) -> &HtmlDom {
        match self {
            Self::Initial(state) => &state.html_dom,
            Self::Drawing(state) => &state.html_dom,
            Self::Ready(state) => &state.html_dom,
        }
    }
}

impl AppState {
    pub fn create(html_dom: HtmlDom) -> Self {
        Self::Initial(InitialState { html_dom })
    }
}
