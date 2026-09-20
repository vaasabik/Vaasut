/// Клип анимации
pub struct AnimationClip {
    pub name: String,
    pub duration: f32,
}

impl AnimationClip {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            duration: 0.0,
        }
    }
}
