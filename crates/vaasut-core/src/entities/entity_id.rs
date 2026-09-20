/// Уникальный идентификатор сущности
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub u64);

impl Entity {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}
