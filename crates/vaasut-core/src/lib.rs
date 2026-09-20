//! Vaasut Core: Базовые типы для всего движка

/// Уникальный идентификатор сущности (Entity ID)
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

/// Измерение, в котором существует объект
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    D2,
    D3,
}

/// Компонент, определяющий, является ли объект 2D или 3D
#[derive(Debug, Clone)]
pub struct DimensionComponent {
    pub dimension: Dimension,
}

impl Default for DimensionComponent {
    fn default() -> Self {
        Self { dimension: Dimension::D3 }
    }
}
