/// Состояние сенсорного экрана
pub struct TouchState {
    pub touches: Vec<(f32, f32)>,
}

impl TouchState {
    pub fn new() -> Self {
        Self { touches: Vec::new() }
    }
}
