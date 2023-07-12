use crate::geometry::Point;

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
}

pub struct InitialState<View: Clone> {
    pub view: View,
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
    pub view: View,
    pub drawings: [Drawing; 8],
}

impl<View: Clone + WithStudent> DrawingState<View> {
    pub fn create(state: &InitialState<View>) -> Self {
        let student = state.get_student();
        let view = state.view.clone();
        Self {
            student,
            label_index: 0,
            view,
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
    #[allow(dead_code)]
    student: String,
    pub view: View,
}

impl<View: Clone> ReadyState<View> {
    pub fn create(state: &DrawingState<View>) -> Self {
        let student = state.student.clone();
        let view = state.view.clone();
        Self { student, view }
    }
}

pub struct SavedState<View> {
    pub view: View,
}

impl<View: Clone> SavedState<View> {
    pub fn create(state: &ReadyState<View>) -> Self {
        Self {
            view: state.view.clone(),
        }
    }
}

pub enum AppState<View: Clone + WithStudent> {
    Initial(InitialState<View>),
    Drawing(DrawingState<View>),
    Ready(ReadyState<View>),
    Saved(SavedState<View>),
}

impl<View: Clone + WithStudent> AppState<View> {
    pub fn create(view: View) -> Self {
        Self::Initial(InitialState { view })
    }

    pub fn drawing_expected_mut(&mut self) -> &mut DrawingState<View> {
        match self {
            AppState::Initial(_) => panic!("unexpected state"),
            AppState::Drawing(state) => state,
            AppState::Ready(_) => panic!("unexpected state"),
            AppState::Saved(_) => panic!("unexpected state"),
        }
    }
}
