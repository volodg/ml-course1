use web_sys::{DomRect, MouseEvent, TouchEvent};

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl From<MouseEvent> for Point {
    fn from(event: MouseEvent) -> Self {
        Self {
            x: event.offset_x(),
            y: event.offset_y(),
        }
    }
}

impl TryFrom<TouchEvent> for Point {
    type Error = ();

    fn try_from(event: TouchEvent) -> Result<Self, Self::Error> {
        match event.touches().get(0) {
            Some(touch) => Ok(Self {
                x: touch.screen_x(),
                y: touch.screen_y(),
            }),
            None => Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl From<DomRect> for Rect {
    fn from(rect: DomRect) -> Self {
        Self {
            x: rect.x() as i32,
            y: rect.y() as i32,
            width: rect.width() as i32,
            height: rect.width() as i32,
        }
    }
}
