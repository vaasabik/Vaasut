//! Vaasut Math: Вектора, матрицы, трансформации для 2D и 3D

// Импортируем базовые типы из glam
pub use glam::{Vec2, Vec3, Mat3, Mat4, Quat};

/// Трансформ для 3D объектов
#[derive(Debug, Clone, Copy)]
pub struct Transform3D {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform3D {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Transform3D {
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
}

/// Трансформ для 2D объектов
#[derive(Debug, Clone, Copy)]
pub struct Transform2D {
    pub position: Vec2,
    pub rotation: f32, // Угол в радианах
    pub scale: Vec2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

impl Transform2D {
    pub fn matrix(&self) -> Mat3 {
        // Создаем матрицу для 2D трансформаций
        let mut mat = Mat3::IDENTITY;
        // Здесь можно добавить логику масштабирования и поворота
        mat
    }
}

/// Универсальный трансформ, который может быть и 2D, и 3D
#[derive(Debug, Clone, Copy)]
pub enum Transform {
    D2(Transform2D),
    D3(Transform3D),
}
