//! Vaasut Core: Базовые типы и структуры

pub mod entities;
pub mod errors;
pub mod events;
pub mod logging;
pub mod primitives;

pub use entities::Entity;
pub use primitives::{Name, Dimension};
