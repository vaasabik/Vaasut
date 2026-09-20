use crate::types::vectors::{Vec2, Vec3};

/// 2D трансформ
#[derive(Debug, Clone, Copy)]
pub struct Transform2D {
    pub position: Vec2,
    pub rotation: f32,
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

/// 3D трансформ
#[derive(Debug, Clone, Copy)]
pub struct Transform3D {
    pub position: Vec3,
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_z: f32,
    pub scale: Vec3,
}

impl Default for Transform3D {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            scale: Vec3::ONE,
        }
    }
}
