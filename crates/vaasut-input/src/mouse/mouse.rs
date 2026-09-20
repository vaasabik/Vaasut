/// Состояние мыши
pub struct MouseState {
    pub x: f32,
    pub y: f32,
    pub left_button: bool,
    pub right_button: bool,
}

impl MouseState {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, left_button: false, right_button: false }
    }
}
