//! Vaasut Physics: Физика для 2D и 3D

use vaasut_math::{Vec2, Vec3};

/// 2D AABB (Axis-Aligned Bounding Box)
#[derive(Debug, Clone, Copy)]
pub struct Aabb2D {
    pub min: Vec2,
    pub max: Vec2,
}

impl Aabb2D {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Проверяет пересечение двух 2D AABB
    pub fn intersects(&self, other: &Aabb2D) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y
    }

    /// Проверяет, находится ли точка внутри 2D AABB
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y
    }
}

/// 3D AABB (Axis-Aligned Bounding Box)
#[derive(Debug, Clone, Copy)]
pub struct Aabb3D {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb3D {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Проверяет пересечение двух 3D AABB
    pub fn intersects(&self, other: &Aabb3D) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    /// Проверяет, находится ли точка внутри 3D AABB
    pub fn contains_point(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }
}

/// Универсальный коллайдер
#[derive(Debug, Clone, Copy)]
pub enum Collider {
    D2(Aabb2D),
    D3(Aabb3D),
}
