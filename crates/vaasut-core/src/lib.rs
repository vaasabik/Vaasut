//! Vaasut Core: Базовые типы для всего движка

/// Уникальный идентификатор сущности (Entity ID)
/// В будущем можно сделать Generational Arena для переиспользования ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub u64);

impl Entity {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Базовый компонент: Имя объекта
#[derive(Debug, Clone)]
pub struct Name(pub String);
