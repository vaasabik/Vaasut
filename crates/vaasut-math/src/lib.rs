//! Vaasut Math: Математика для игр

pub mod transforms;
pub mod types;
pub mod utils;

pub use types::vectors::{Vec2, Vec3, Vec4};
pub use types::matrices::{Mat3, Mat4};
pub use types::colors::Color;
pub use transforms::{Transform2D, Transform3D};
