//! Компоненты трансформации

use vaasut_math::{Transform2D, Transform3D, Vec2, Vec3, Quat};

/// 2D трансформ для спрайтов и плоских объектов
#[derive(Debug, Clone, Copy)]
pub struct Transform2DComponent {
    pub transform: Transform2D,
}

impl Transform2DComponent {
    pub fn new(transform: Transform2D) -> Self {
        Self { transform }
    }
    
    pub fn from_position(x: f32, y: f32) -> Self {
        Self {
            transform: Transform2D::from_position(Vec2::new(x, y)),
        }
    }
    
    pub fn position(&self) -> Vec2 {
        self.transform.position
    }
    
    pub fn rotation(&self) -> f32 {
        self.transform.rotation
    }
    
    pub fn scale(&self) -> Vec2 {
        self.transform.scale
    }
}

impl Default for Transform2DComponent {
    fn default() -> Self {
        Self {
            transform: Transform2D::default(),
        }
    }
}

/// 3D трансформ для моделей и 3D объектов
#[derive(Debug, Clone, Copy)]
pub struct Transform3DComponent {
    pub transform: Transform3D,
}

impl Transform3DComponent {
    pub fn new(transform: Transform3D) -> Self {
        Self { transform }
    }
    
    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            transform: Transform3D::from_position(Vec3::new(x, y, z)),
        }
    }
    
    pub fn position(&self) -> Vec3 {
        self.transform.position
    }
    
    pub fn rotation(&self) -> Quat {
        self.transform.rotation
    }
    
    pub fn scale(&self) -> Vec3 {
        self.transform.scale
    }
}

impl Default for Transform3DComponent {
    fn default() -> Self {
        Self {
            transform: Transform3D::default(),
        }
    }
}
