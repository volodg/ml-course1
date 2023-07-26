use crate::html::HtmlDom;
use commons::math::Point;
use drawing_commons::models::{DrawingPaths, Features};
use std::cell::RefCell;
use std::rc::Rc;
use web_commons::log;

pub trait DrawingAnalyzer {
    fn subscribe_drawing_updates(&self);
}

impl DrawingAnalyzer for HtmlDom {
    fn subscribe_drawing_updates(&self) {
        let mut sketch_pad = self.sketch_pad.borrow_mut();

        let on_update_callback = Rc::new(RefCell::new(move |drawing: &DrawingPaths<Point>| {
            let point = Point {
                x: drawing.path_count() as f64,
                y: drawing.point_count() as f64,
            };

            log(std::format!("point: {:?}", point).as_str())
        }));

        sketch_pad.set_on_update(on_update_callback)
    }
}
