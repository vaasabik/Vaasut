//! Алиасы типов результатов для удобства

use super::error::VaasutError;

/// Стандартный результат для движка
pub type VaasutResult<T> = Result<T, VaasutError>;

/// Результат без значения (только успех или ошибка)
pub type VaasutOutcome = Result<(), VaasutError>;

/// Результат для опциональных значений
pub type VaasutOption<T> = Option<T>;

/// Расширения для Result
pub trait VaasutResultExt<T> {
    /// Логирует ошибку и возвращает результат
    fn log_error(self) -> Self;
    
    /// Логирует ошибку как предупреждение и возвращает результат
    fn log_warning(self) -> Self;
    
    /// Игнорирует ошибку, но логирует её
    fn ignore_error(self) -> Option<T>;
    
    /// Возвращает значение или значение по умолчанию, логируя ошибку
    fn unwrap_or_log(self, default: T) -> T;
}

impl<T> VaasutResultExt<T> for VaasutResult<T> {
    fn log_error(self) -> Self {
        if let Err(ref err) = self {
            eprintln!("[ERROR] {}", err);
        }
        self
    }
    
    fn log_warning(self) -> Self {
        if let Err(ref err) = self {
            eprintln!("[WARNING] {}", err);
        }
        self
    }
    
    fn ignore_error(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                eprintln!("[ERROR IGNORED] {}", err);
                None
            }
        }
    }
    
    fn unwrap_or_log(self, default: T) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                eprintln!("[ERROR] {} (using default)", err);
                default
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::error::{VaasutError, ErrorCategory};
    
    #[test]
    fn test_result_ext() {
        let ok: VaasutResult<i32> = Ok(42);
        let err: VaasutResult<i32> = Err(VaasutError::new(ErrorCategory::General, "test error"));
        
        assert_eq!(ok.ignore_error(), Some(42));
        assert_eq!(err.ignore_error(), None);
        assert_eq!(err.unwrap_or_log(0), 0);
    }
}
