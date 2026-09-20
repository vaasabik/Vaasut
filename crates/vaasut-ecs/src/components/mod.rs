//! Компоненты для игровых объектов

pub mod common;
pub mod transform;
pub mod sprite;
pub mod mesh;
pub mod camera;
pub mod light;
pub mod physics;
pub mod state;

// Реэкспортируем часто используемые типы
pub use common::{Visible, Tag, RenderOrder, Static, Active};
pub use transform::{Transform2DComponent, Transform3DComponent};
pub use sprite::{Sprite, SpriteSheet, SpriteAnimation, TextureId};
pub use mesh::{MeshComponent, MeshId, MaterialId, MaterialComponent, Primitive, PrimitiveType};
pub use camera::{Camera2D, Camera3D, RenderMode};
pub use light::{Light, LightType};
pub use physics::{Collider2D, Collider3D, RigidBody2D, RigidBody3D};
pub use state::{StateMachineComponent, StateMachineSystem};
