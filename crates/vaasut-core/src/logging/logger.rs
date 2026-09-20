//! Система логирования

use crate::errors::{VaasutError, ErrorSeverity};

/// Уровень логирования
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Отладочные сообщения
    Debug,
    /// Информационные сообщения
    Info,
    /// Предупреждения
    Warning,
    /// Ошибки
    Error,
}

impl LogLevel {
    pub fn prefix(&self) -> &'static str {
        match self {
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARN]",
            LogLevel::Error => "[ERROR]",
        }
    }
}

/// Логгер движка
pub struct Logger {
    /// Минимальный уровень логирования
    pub min_level: LogLevel,
    /// Включено ли логирование в консоль
    pub console_output: bool,
}

impl Logger {
    /// Создаёт новый логгер
    pub fn new() -> Self {
        Self {
            min_level: LogLevel::Info,
            console_output: true,
        }
    }
    
    /// Устанавливает минимальный уровень
    pub fn with_min_level(mut self, level: LogLevel) -> Self {
        self.min_level = level;
        self
    }
    
    /// Логирует сообщение
    pub fn log(&self, level: LogLevel, message: &str) {
        if level >= self.min_level && self.console_output {
            println!("{} {}", level.prefix(), message);
        }
    }
    
    /// Логирует отладочное сообщение
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    
    /// Логирует информационное сообщение
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    
    /// Логирует предупреждение
    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }
    
    /// Логирует ошибку
    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    
    /// Логирует ошибку движка
    pub fn log_vaasut_error(&self, error: &VaasutError) {
        let level = match error.severity {
            ErrorSeverity::Warning => LogLevel::Warning,
            ErrorSeverity::Error => LogLevel::Error,
            ErrorSeverity::Critical => LogLevel::Error,
        };
        
        self.log(level, &error.to_string());
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

/// Глобальный логгер (статический)
pub fn log_info(message: &str) {
    println!("[INFO] {}", message);
}

pub fn log_warning(message: &str) {
    eprintln!("[WARN] {}", message);
}

pub fn log_error(message: &str) {
    eprintln!("[ERROR] {}", message);
}

pub fn log_debug(message: &str) {
    println!("[DEBUG] {}", message);
}
