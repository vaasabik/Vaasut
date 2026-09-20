//! Ошибки рендеринга

use super::error::{VaasutError, ErrorCategory};

/// Ошибки рендеринга
#[derive(Debug, Clone)]
pub enum RenderError {
    /// Не удалось создать графическое устройство
    DeviceCreationFailed(String),
    /// Не удалось скомпилировать шейдер
    ShaderCompilationFailed { shader_name: String, error_message: String },
    /// Не удалось создать конвейер
    PipelineCreationFailed(String),
    /// Не удалось создать текстуру
    TextureCreationFailed(String),
    /// Не удалось создать буфер
    BufferCreationFailed(String),
    /// Не удалось отрисовать кадр
    RenderPassFailed(String),
    /// Поверхность не поддерживается
    UnsupportedSurfaceFormat(String),
    /// Ошибка окна
    WindowError(String),
}

impl RenderError {
    /// Преобразует в VaasutError
    pub fn to_vaasut_error(&self) -> VaasutError {
        match self {
            RenderError::DeviceCreationFailed(msg) => {
                VaasutError::new(ErrorCategory::Rendering, format!("Device creation failed: {}", msg))
            }
            RenderError::ShaderCompilationFailed { shader_name, error_message } => {
                VaasutError::new(
                    ErrorCategory::Rendering,
                    format!("Shader '{}' compilation failed: {}", shader_name, error_message)
                )
            }
            RenderError::PipelineCreationFailed(msg) => {
                VaasutError::new(ErrorCategory::Rendering, format!("Pipeline creation failed: {}", msg))
            }
            RenderError::TextureCreationFailed(msg) => {
                VaasutError::new(ErrorCategory::Rendering, format!("Texture creation failed: {}", msg))
            }
            RenderError::BufferCreationFailed(msg) => {
                VaasutError::new(ErrorCategory::Rendering, format!("Buffer creation failed: {}", msg))
            }
            RenderError::RenderPassFailed(msg) => {
                VaasutError::new(ErrorCategory::Rendering, format!("Render pass failed: {}", msg))
            }
            RenderError::UnsupportedSurfaceFormat(format) => {
                VaasutError::new(
                    ErrorCategory::Rendering,
                    format!("Unsupported surface format: {}", format)
                )
            }
            RenderError::WindowError(msg) => {
                VaasutError::new(ErrorCategory::Rendering, format!("Window error: {}", msg))
            }
        }
    }
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::DeviceCreationFailed(msg) => write!(f, "Device creation failed: {}", msg),
            RenderError::ShaderCompilationFailed { shader_name, error_message } => {
                write!(f, "Shader '{}' failed: {}", shader_name, error_message)
            }
            RenderError::PipelineCreationFailed(msg) => write!(f, "Pipeline failed: {}", msg),
            RenderError::TextureCreationFailed(msg) => write!(f, "Texture failed: {}", msg),
            RenderError::BufferCreationFailed(msg) => write!(f, "Buffer failed: {}", msg),
            RenderError::RenderPassFailed(msg) => write!(f, "Render pass failed: {}", msg),
            RenderError::UnsupportedSurfaceFormat(format) => {
                write!(f, "Unsupported format: {}", format)
            }
            RenderError::WindowError(msg) => write!(f, "Window error: {}", msg),
        }
    }
}

impl std::error::Error for RenderError {}

impl From<RenderError> for VaasutError {
    fn from(err: RenderError) -> Self {
        err.to_vaasut_error()
    }
}
