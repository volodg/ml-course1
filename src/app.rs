use crate::geometry::Point;
use crate::html::HtmlDom;

const LABELS: [&str; 8] = [
    "car", "fish", "house", "tree", "bicycle", "guitar", "pencil", "clock",
];

pub struct AppState {
    pub student: Option<String>,
    pub label_index: usize,
    pub html_dom: HtmlDom,
    pub pressed: bool,
    pub paths: Vec<Vec<Point>>,
}

impl AppState {
    pub fn add_point(&mut self, point: Point) {
        let size = self.paths.len();
        self.paths[size - 1].push(point);
    }

    pub fn undo(&mut self) {
        while let Some(last) = self.paths.last() {
            if last.is_empty() {
                self.paths.pop();
            } else {
                break;
            }
        }
        self.paths.pop();
    }

    pub fn get_current_label(&self) -> &str {
        LABELS[self.label_index]
    }

    pub fn increment_index(&mut self) {
        self.label_index = (self.label_index + 1) % LABELS.len();
    }
}
