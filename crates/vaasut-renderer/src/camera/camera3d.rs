//! 3D камера (перспективная)

use vaasut_math::{Vec3, Mat4};

/// 3D камера для рендеринга
#[derive(Debug, Clone)]
pub struct Camera3DView {
    /// Позиция камеры
    pub position: Vec3,
    /// Куда смотрит камера
    pub target: Vec3,
    /// Вектор "вверх"
    pub up: Vec3,
    /// Поле зрения в радианах
    pub fov: f32,
    /// Соотношение сторон
    pub aspect: f32,
    /// Ближняя плоскость
    pub near: f32,
    /// Дальняя плоскость
    pub far: f32,
}

impl Camera3DView {
    pub fn new(aspect: f32) -> Self {
        Self {
            position: Vec3::new(0.0, 5.0, 10.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fov: 60.0_f32.to_radians(),
            aspect,
            near: 0.1,
            far: 1000.0,
        }
    }
    
    /// Возвращает матрицу вида
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }
    
    /// Возвращает матрицу проекции (перспективная)
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }
}
