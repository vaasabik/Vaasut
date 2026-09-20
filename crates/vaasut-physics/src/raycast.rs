//! Рейкасты (лучи)

use vaasut_math::{Vec2, Vec3};

/// 2D луч
pub struct Ray2D {
    pub origin: Vec2,
    pub direction: Vec2,
}

/// 3D луч
pub struct Ray3D {
    pub origin: Vec3,
    pub direction: Vec3,
}
