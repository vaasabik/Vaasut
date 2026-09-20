//! Система логирования

pub mod logger;
pub use logger::{Logger, LogLevel, log_info, log_warning, log_error, log_debug};
