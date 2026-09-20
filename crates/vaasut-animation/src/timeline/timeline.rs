/// Таймлайн анимации
pub struct Timeline {
    pub current_time: f32,
    pub is_playing: bool,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            current_time: 0.0,
            is_playing: false,
        }
    }
}
