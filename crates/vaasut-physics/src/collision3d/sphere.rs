use vaasut_math::Vec3;

/// 3D сфера
#[derive(Debug, Clone, Copy)]
pub struct Sphere3D {
    pub center: Vec3,
    pub radius: f32,
}
