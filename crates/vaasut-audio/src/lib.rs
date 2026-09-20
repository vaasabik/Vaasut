//! Vaasut Audio: Воспроизведение звуков и музыки

pub struct AudioEngine {
    pub volume: f32,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self { volume: 1.0 }
    }
}
