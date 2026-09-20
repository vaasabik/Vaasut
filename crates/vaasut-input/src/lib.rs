//! Vaasut Input: Управление клавиатурой, мышью, тачем

pub struct InputState {
    pub mouse_position: (f32, f32),
    pub mouse_down: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_position: (0.0, 0.0),
            mouse_down: false,
        }
    }
}
