//! Система ошибок движка
//! 
//! Предоставляет типизированные ошибки для всех подсистем движка.
//! Каждая подсистема имеет свой тип ошибок, что позволяет точно
//! определить причину сбоя и принять соответствующие меры.

pub mod error;
pub mod result;
pub mod render;
pub mod asset;
pub mod scene;

pub use error::{VaasutError, ErrorSeverity, ErrorCategory};
pub use result::VaasutResult;
pub use render::RenderError;
pub use asset::AssetError;
pub use scene::SceneError;
