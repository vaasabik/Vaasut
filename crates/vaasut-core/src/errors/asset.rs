//! Ошибки загрузки ресурсов

use super::error::{VaasutError, ErrorCategory};

/// Ошибки ассетов
#[derive(Debug, Clone)]
pub enum AssetError {
    /// Файл не найден
    FileNotFound { path: String },
    /// Неверный формат файла
    InvalidFormat { path: String, expected: String, actual: String },
    /// Ошибка чтения файла
    ReadFailed { path: String, reason: String },
    /// Ошибка парсинга
    ParseFailed { path: String, reason: String },
    /// Ассет слишком большой
    TooLarge { path: String, size_bytes: u64, max_bytes: u64 },
    /// Ассет ещё не загружен
    NotLoaded { asset_id: u64 },
    /// Ошибка декодирования изображения
    ImageDecodeFailed { path: String },
    /// Ошибка декодирования звука
    AudioDecodeFailed { path: String },
    /// Ошибка декодирования модели
    ModelDecodeFailed { path: String },
}

impl AssetError {
    /// Преобразует в VaasutError
    pub fn to_vaasut_error(&self) -> VaasutError {
        match self {
            AssetError::FileNotFound { path } => {
                VaasutError::new(ErrorCategory::Asset, format!("File not found: {}", path))
            }
            AssetError::InvalidFormat { path, expected, actual } => {
                VaasutError::new(
                    ErrorCategory::Asset,
                    format!("Invalid format for '{}': expected {}, got {}", path, expected, actual)
                )
            }
            AssetError::ReadFailed { path, reason } => {
                VaasutError::new(ErrorCategory::Asset, format!("Failed to read '{}': {}", path, reason))
            }
            AssetError::ParseFailed { path, reason } => {
                VaasutError::new(ErrorCategory::Asset, format!("Failed to parse '{}': {}", path, reason))
            }
            AssetError::TooLarge { path, size_bytes, max_bytes } => {
                VaasutError::new(
                    ErrorCategory::Asset,
                    format!("Asset '{}' too large: {} bytes (max: {})", path, size_bytes, max_bytes)
                )
            }
            AssetError::NotLoaded { asset_id } => {
                VaasutError::new(ErrorCategory::Asset, format!("Asset {} not loaded", asset_id))
            }
            AssetError::ImageDecodeFailed { path } => {
                VaasutError::new(ErrorCategory::Asset, format!("Failed to decode image: {}", path))
            }
            AssetError::AudioDecodeFailed { path } => {
                VaasutError::new(ErrorCategory::Asset, format!("Failed to decode audio: {}", path))
            }
            AssetError::ModelDecodeFailed { path } => {
                VaasutError::new(ErrorCategory::Asset, format!("Failed to decode model: {}", path))
            }
        }
    }
    
    /// Создаёт ошибку "файл не найден"
    pub fn file_not_found(path: impl Into<String>) -> Self {
        AssetError::FileNotFound { path: path.into() }
    }
    
    /// Создаёт ошибку "неверный формат"
    pub fn invalid_format(path: impl Into<String>, expected: impl Into<String>, actual: impl Into<String>) -> Self {
        AssetError::InvalidFormat {
            path: path.into(),
            expected: expected.into(),
            actual: actual.into(),
        }
    }
}

impl std::fmt::Display for AssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetError::FileNotFound { path } => write!(f, "File not found: {}", path),
            AssetError::InvalidFormat { path, expected, actual } => {
                write!(f, "Invalid format for '{}': expected {}, got {}", path, expected, actual)
            }
            AssetError::ReadFailed { path, reason } => {
                write!(f, "Failed to read '{}': {}", path, reason)
            }
            AssetError::ParseFailed { path, reason } => {
                write!(f, "Failed to parse '{}': {}", path, reason)
            }
            AssetError::TooLarge { path, size_bytes, max_bytes } => {
                write!(f, "Asset '{}' too large: {} bytes", path, size_bytes)
            }
            AssetError::NotLoaded { asset_id } => write!(f, "Asset {} not loaded", asset_id),
            AssetError::ImageDecodeFailed { path } => write!(f, "Image decode failed: {}", path),
            AssetError::AudioDecodeFailed { path } => write!(f, "Audio decode failed: {}", path),
            AssetError::ModelDecodeFailed { path } => write!(f, "Model decode failed: {}", path),
        }
    }
}

impl std::error::Error for AssetError {}

impl From<AssetError> for VaasutError {
    fn from(err: AssetError) -> Self {
        err.to_vaasut_error()
    }
}
