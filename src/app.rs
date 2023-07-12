use crate::geometry::Point;
use crate::html::HtmlDom;

struct Drawing {
    label: &'static str,
    paths: Vec<Vec<Point>>,
}

pub struct AppState {
    pub student: Option<String>,
    pub label_index: usize,
    pub html_dom: HtmlDom,
    pub pressed: bool,
    drawings: [Drawing; 8],
}

impl AppState {
    pub fn create(html_dom: HtmlDom) -> Self {
        Self {
            student: None,
            label_index: 0,
            html_dom,
            pressed: false,
            drawings: [
                Drawing {
                    label: "car",
                    paths: vec![],
                },
                Drawing {
                    label: "fish",
                    paths: vec![],
                },
                Drawing {
                    label: "house",
                    paths: vec![],
                },
                Drawing {
                    label: "tree",
                    paths: vec![],
                },
                Drawing {
                    label: "bicycle",
                    paths: vec![],
                },
                Drawing {
                    label: "guitar",
                    paths: vec![],
                },
                Drawing {
                    label: "pencil",
                    paths: vec![],
                },
                Drawing {
                    label: "clock",
                    paths: vec![],
                },
            ],
        }
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

    pub fn increment_index(&mut self) {
        self.label_index = (self.label_index + 1) % self.curr_path_mut().len();
    }
}
