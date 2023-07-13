use crate::geometry::Point;
use js_sys::Date;
use commons::utils::SomeExt;

const DRAWING_SIZE: usize = 8;

#[derive(Clone)]
pub struct Drawing {
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

    pub fn get_label(&self) -> &str {
        self.label
    }

    pub fn get_paths(&self) -> &Vec<Vec<Point>> {
        &self.paths
    }
}

pub struct InitialState<View: Clone> {
    view: View,
    session: u64,
}

impl<View: Clone> InitialState<View> {
    pub fn create(view: View) -> Self {
        let session = Date::now() as u64;
        Self { view, session, }
    }

    pub fn get_view(&self) -> &View {
        &self.view
    }
}

pub trait WithStudent {
    fn get_student(&self) -> String;
}

impl<View: Clone + WithStudent> InitialState<View> {
    pub fn get_student(&self) -> String {
        self.view.get_student()
    }
}

pub struct DrawingState<View> {
    pub student: String,
    label_index: usize,
    view: View,
    session: u64,
    pub drawings: [Drawing; DRAWING_SIZE],
}

impl<View: Clone + WithStudent> DrawingState<View> {
    pub fn create(state: &InitialState<View>) -> Self {
        let student = state.get_student();
        let view = state.view.clone();
        let session = state.session.clone();
        Self {
            student,
            label_index: 0,
            view,
            session,
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

    pub fn get_view(&self) -> &View {
        &self.view
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

pub struct ReadyState<View: Clone> {
    view: View,
    pub session: u64,
    pub student: String,
    pub drawings: [Drawing; DRAWING_SIZE],
}

impl<View: Clone> ReadyState<View> {
    pub fn create(state: &DrawingState<View>) -> Self {
        let view = state.view.clone();
        let session = state.session;
        let student = state.student.clone();
        let drawings = state.drawings.clone();
        Self {
            view,
            session,
            student,
            drawings,
        }
    }

    pub fn get_view(&self) -> &View {
        &self.view
    }
}

pub struct SavedState<View> {
    view: View,
}

impl<View: Clone> SavedState<View> {
    pub fn create(state: &ReadyState<View>) -> Self {
        Self {
            view: state.view.clone(),
        }
    }

    pub fn get_view(&self) -> &View {
        &self.view
    }
}

pub enum AppState<View: Clone + WithStudent> {
    Initial(InitialState<View>),
    Drawing(DrawingState<View>),
    Ready(ReadyState<View>),
    Saved(SavedState<View>),
}

impl<View: Clone + WithStudent> AppState<View> {
    pub fn create(state: InitialState<View>) -> Self {
        Self::Initial(state)
    }

    pub fn drawing_expected_mut(&mut self) -> Option<&mut DrawingState<View>> {
        match self {
            AppState::Initial(_) => None,
            AppState::Drawing(state) => state.some(),
            AppState::Ready(_) => None,
            AppState::Saved(_) => None,
        }
    }
}
