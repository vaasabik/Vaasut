//! Базовый тип ошибки движка

use std::fmt;

/// Категория ошибки — к какой подсистеме относится
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Общая ошибка
    General,
    /// Ошибка рендеринга
    Rendering,
    /// Ошибка загрузки ресурсов
    Asset,
    /// Ошибка сцены
    Scene,
    /// Ошибка ввода/вывода
    Io,
    /// Ошибка сериализации
    Serialization,
    /// Ошибка физики
    Physics,
    /// Ошибка аудио
    Audio,
    /// Ошибка скриптинга
    Script,
    /// Ошибка сети
    Network,
}

impl ErrorCategory {
    pub fn name(&self) -> &'static str {
        match self {
            ErrorCategory::General => "General",
            ErrorCategory::Rendering => "Rendering",
            ErrorCategory::Asset => "Asset",
            ErrorCategory::Scene => "Scene",
            ErrorCategory::Io => "IO",
            ErrorCategory::Serialization => "Serialization",
            ErrorCategory::Physics => "Physics",
            ErrorCategory::Audio => "Audio",
            ErrorCategory::Script => "Script",
            ErrorCategory::Network => "Network",
        }
    }
}

/// Степень серьёзности ошибки
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorSeverity {
    /// Предупреждение — можно продолжить работу
    Warning,
    /// Ошибка — операция не удалась, но движок может работать
    Error,
    /// Критическая ошибка — движок должен остановиться
    Critical,
}

impl ErrorSeverity {
    pub fn name(&self) -> &'static str {
        match self {
            ErrorSeverity::Warning => "WARNING",
            ErrorSeverity::Error => "ERROR",
            ErrorSeverity::Critical => "CRITICAL",
        }
    }
}

/// Основная ошибка движка
#[derive(Debug, Clone)]
pub struct VaasutError {
    /// Категория ошибки
    pub category: ErrorCategory,
    /// Степень серьёзности
    pub severity: ErrorSeverity,
    /// Краткое описание
    pub message: String,
    /// Детальная информация (для отладки)
    pub details: Option<String>,
    /// Исходная ошибка (если ошибка была преобразована)
    pub source: Option<String>,
}

impl VaasutError {
    /// Создаёт новую ошибку
    pub fn new(category: ErrorCategory, message: impl Into<String>) -> Self {
        Self {
            category,
            severity: ErrorSeverity::Error,
            message: message.into(),
            details: None,
            source: None,
        }
    }
    
    /// Создаёт ошибку с серьёзностью
    pub fn with_severity(
        category: ErrorCategory,
        severity: ErrorSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            category,
            severity,
            message: message.into(),
            details: None,
            source: None,
        }
    }
    
    /// Добавляет детали
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
    
    /// Добавляет исходную ошибку
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
    
    /// Является ли критической?
    pub fn is_critical(&self) -> bool {
        self.severity == ErrorSeverity::Critical
    }
    
    /// Является ли предупреждением?
    pub fn is_warning(&self) -> bool {
        self.severity == ErrorSeverity::Warning
    }
    
    /// Преобразует в предупреждение
    pub fn as_warning(mut self) -> Self {
        self.severity = ErrorSeverity::Warning;
        self
    }
    
    /// Преобразует в критическую
    pub fn as_critical(mut self) -> Self {
        self.severity = ErrorSeverity::Critical;
        self
    }
}

impl fmt::Display for VaasutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}:{}] {}",
            self.severity.name(),
            self.category.name(),
            self.message
        )?;
        
        if let Some(details) = &self.details {
            write!(f, " | {}", details)?;
        }
        
        if let Some(source) = &self.source {
            write!(f, " | Caused by: {}", source)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for VaasutError {}

// Реализуем конвертацию из стандартных ошибок

impl From<std::io::Error> for VaasutError {
    fn from(err: std::io::Error) -> Self {
        VaasutError::new(ErrorCategory::Io, format!("IO error: {}", err))
            .with_source(err.to_string())
    }
}

impl From<std::fmt::Error> for VaasutError {
    fn from(err: std::fmt::Error) -> Self {
        VaasutError::new(ErrorCategory::General, format!("Format error: {}", err))
            .with_source(err.to_string())
    }
}

// Макросы для удобного создания ошибок

/// Создаёт ошибку рендеринга
#[macro_export]
macro_rules! render_error {
    ($msg:expr) => {
        VaasutError::new(ErrorCategory::Rendering, $msg)
    };
    ($msg:expr, $($arg:tt)*) => {
        VaasutError::new(ErrorCategory::Rendering, format!($msg, $($arg)*))
    };
}

/// Создаёт ошибку ассетов
#[macro_export]
macro_rules! asset_error {
    ($msg:expr) => {
        VaasutError::new(ErrorCategory::Asset, $msg)
    };
    ($msg:expr, $($arg:tt)*) => {
        VaasutError::new(ErrorCategory::Asset, format!($msg, $($arg)*))
    };
}

/// Создаёт ошибку сцены
#[macro_export]
macro_rules! scene_error {
    ($msg:expr) => {
        VaasutError::new(ErrorCategory::Scene, $msg)
    };
    ($msg:expr, $($arg:tt)*) => {
        VaasutError::new(ErrorCategory::Scene, format!($msg, $($arg)*))
    };
}

/// Создаёт критическую ошибку
#[macro_export]
macro_rules! critical_error {
    ($category:expr, $msg:expr) => {
        VaasutError::with_severity($category, ErrorSeverity::Critical, $msg)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let err = VaasutError::new(ErrorCategory::Rendering, "Shader compilation failed");
        assert_eq!(err.category, ErrorCategory::Rendering);
        assert_eq!(err.severity, ErrorSeverity::Error);
        assert!(err.message.contains("Shader"));
    }
    
    #[test]
    fn test_error_with_details() {
        let err = VaasutError::new(ErrorCategory::Asset, "File not found")
            .with_details("Path: assets/textures/player.png")
            .with_source("std::io::Error");
        
        assert!(err.details.is_some());
        assert!(err.source.is_some());
    }
    
    #[test]
    fn test_error_display() {
        let err = VaasutError::new(ErrorCategory::Scene, "Entity not found");
        let display = format!("{}", err);
        assert!(display.contains("ERROR"));
        assert!(display.contains("Scene"));
        assert!(display.contains("Entity not found"));
    }
    
    #[test]
    fn test_error_severity() {
        let warning = VaasutError::with_severity(
            ErrorCategory::General,
            ErrorSeverity::Warning,
            "Low memory"
        );
        assert!(warning.is_warning());
        assert!(!warning.is_critical());
        
        let critical = warning.as_critical();
        assert!(critical.is_critical());
    }
}
