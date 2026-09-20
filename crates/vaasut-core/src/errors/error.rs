/// Базовый тип ошибки движка
#[derive(Debug)]
pub enum VaasutError {
    NotFound(String),
    InvalidState(String),
    IoError(String),
}

impl std::fmt::Display for VaasutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VaasutError::NotFound(msg) => write!(f, "Not found: {}", msg),
            VaasutError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            VaasutError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}
