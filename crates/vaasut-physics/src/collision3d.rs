//! 3D коллизии

use vaasut_math::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Aabb3D {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb3D {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
    
    pub fn intersects(&self, other: &Aabb3D) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
}
