use commons::geometry::Point2D;
use commons::utils::OkExt;
use web_sys::{MouseEvent, TouchEvent};

pub fn convert_mouse_event_into_point(event: MouseEvent) -> Point2D {
    Point2D {
        x: event.offset_x() as f64,
        y: event.offset_y() as f64,
    }
}

pub fn try_convert_touch_event_into_point(event: TouchEvent) -> Result<Point2D, ()> {
    match event.touches().get(0) {
        Some(touch) => Point2D {
            x: touch.client_x() as f64,
            y: touch.client_y() as f64,
        }
        .ok(),
        None => Err(()),
    }
}
