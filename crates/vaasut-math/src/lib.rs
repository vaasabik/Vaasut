//! Vaasut Math: Математика для 2D и 3D

// Объявляем подмодули
pub mod vec2;
pub mod vec3;
pub mod transform;
pub mod matrix;

// Реэкспортируем основные типы для удобства использования
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use transform::{Transform2D, Transform3D};
pub use matrix::{Mat3, Mat4};
