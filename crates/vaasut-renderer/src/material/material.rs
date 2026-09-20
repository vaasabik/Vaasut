//! Материал — определяет, как объект выглядит

use vaasut_math::Color;

/// Материал для рендеринга
#[derive(Debug, Clone)]
pub struct Material {
    /// Основной цвет
    pub color: Color,
    /// Прозрачность
    pub opacity: f32,
}

impl Material {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            opacity: 1.0,
        }
    }
    
    pub fn to_array(&self) -> [f32; 4] {
        [self.color.r, self.color.g, self.color.b, self.opacity]
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(Color::WHITE)
    }
}
