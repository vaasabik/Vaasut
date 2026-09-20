/// Матрица 3x3
#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub data: [[f32; 3]; 3],
}

impl Mat3 {
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
}
