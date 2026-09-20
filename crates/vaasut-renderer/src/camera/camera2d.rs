//! 2D камера (ортографическая)

use vaasut_math::{Vec2, Mat4};

/// 2D камера для рендеринга
#[derive(Debug, Clone)]
pub struct Camera2DView {
    /// Позиция камеры
    pub position: Vec2,
    /// Масштаб
    pub zoom: f32,
    /// Размер вьюпорта (ширина, высота)
    pub viewport_size: Vec2,
}

impl Camera2DView {
    pub fn new(viewport_size: Vec2) -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            viewport_size,
        }
    }
    
    /// Возвращает матрицу вида
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::from_translation(Vec3::new(-self.position.x, -self.position.y, 0.0))
    }
    
    /// Возвращает матрицу проекции (ортографическая)
    pub fn projection_matrix(&self) -> Mat4 {
        let half_width = self.viewport_size.x / 2.0 * self.zoom;
        let half_height = self.viewport_size.y / 2.0 * self.zoom;
        
        Mat4::orthographic_rh(
            -half_width,
            half_width,
            -half_height,
            half_height,
            -1.0,
            1.0,
        )
    }
}

use vaasut_math::Vec3;
