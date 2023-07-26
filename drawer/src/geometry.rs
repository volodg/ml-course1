use web_sys::DomRect;

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
