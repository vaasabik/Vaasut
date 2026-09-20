/// Звуковой движок
pub struct AudioEngine {
    pub master_volume: f32,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self { master_volume: 1.0 }
    }
}
