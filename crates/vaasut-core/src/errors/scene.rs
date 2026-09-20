//! Ошибки сцены

use super::error::{VaasutError, ErrorCategory};

/// Ошибки сцены
#[derive(Debug, Clone)]
pub enum SceneError {
    /// Сущность не найдена
    EntityNotFound { entity_id: u64 },
    /// Компонент не найден у сущности
    ComponentNotFound { entity_id: u64, component_name: String },
    /// Сцена не найдена
    SceneNotFound { scene_name: String },
    /// Ошибка сериализации сцены
    SerializationFailed { scene_name: String, reason: String },
    /// Ошибка десериализации сцены
    DeserializationFailed { scene_name: String, reason: String },
    /// Попытка удалить защищённую сущность
    ProtectedEntity { entity_id: u64 },
}

impl SceneError {
    /// Преобразует в VaasutError
    pub fn to_vaasut_error(&self) -> VaasutError {
        match self {
            SceneError::EntityNotFound { entity_id } => {
                VaasutError::new(ErrorCategory::Scene, format!("Entity {} not found", entity_id))
            }
            SceneError::ComponentNotFound { entity_id, component_name } => {
                VaasutError::new(
                    ErrorCategory::Scene,
                    format!("Component '{}' not found on entity {}", component_name, entity_id)
                )
            }
            SceneError::SceneNotFound { scene_name } => {
                VaasutError::new(ErrorCategory::Scene, format!("Scene '{}' not found", scene_name))
            }
            SceneError::SerializationFailed { scene_name, reason } => {
                VaasutError::new(
                    ErrorCategory::Scene,
                    format!("Failed to serialize scene '{}': {}", scene_name, reason)
                )
            }
            SceneError::DeserializationFailed { scene_name, reason } => {
                VaasutError::new(
                    ErrorCategory::Scene,
                    format!("Failed to deserialize scene '{}': {}", scene_name, reason)
                )
            }
            SceneError::ProtectedEntity { entity_id } => {
                VaasutError::new(
                    ErrorCategory::Scene,
                    format!("Cannot modify protected entity {}", entity_id)
                )
            }
        }
    }
    
    /// Создаёт ошибку "сущность не найдена"
    pub fn entity_not_found(entity_id: u64) -> Self {
        SceneError::EntityNotFound { entity_id }
    }
    
    /// Создаёт ошибку "компонент не найден"
    pub fn component_not_found(entity_id: u64, component_name: impl Into<String>) -> Self {
        SceneError::ComponentNotFound {
            entity_id,
            component_name: component_name.into(),
        }
    }
}

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SceneError::EntityNotFound { entity_id } => write!(f, "Entity {} not found", entity_id),
            SceneError::ComponentNotFound { entity_id, component_name } => {
                write!(f, "Component '{}' not found on entity {}", component_name, entity_id)
            }
            SceneError::SceneNotFound { scene_name } => write!(f, "Scene '{}' not found", scene_name),
            SceneError::SerializationFailed { scene_name, reason } => {
                write!(f, "Serialization failed for '{}': {}", scene_name, reason)
            }
            SceneError::DeserializationFailed { scene_name, reason } => {
                write!(f, "Deserialization failed for '{}': {}", scene_name, reason)
            }
            SceneError::ProtectedEntity { entity_id } => {
                write!(f, "Entity {} is protected", entity_id)
            }
        }
    }
}

impl std::error::Error for SceneError {}

impl From<SceneError> for VaasutError {
    fn from(err: SceneError) -> Self {
        err.to_vaasut_error()
    }
}
