//! Идентификатор сущности с генерацией
//! 
//! Generational Index защищает от ошибок "использование после удаления".
//! Когда сущность удаляется и создаётся новая, у неё будет другой индекс поколения.

/// Индекс сущности в массиве
pub type EntityIndex = u32;
/// Поколение сущности (увеличивается при переиспользовании индекса)
pub type EntityGeneration = u32;

/// Уникальный идентификатор сущности
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    /// Индекс в хранилище
    pub index: EntityIndex,
    /// Поколение (защита от устаревших ссылок)
    pub generation: EntityGeneration,
}

impl Entity {
    /// Создаёт новую сущность
    pub fn new(index: EntityIndex, generation: EntityGeneration) -> Self {
        Self { index, generation }
    }
    
    /// Возвращает индекс
    pub fn index(&self) -> EntityIndex {
        self.index
    }
    
    /// Возвращает поколение
    pub fn generation(&self) -> EntityGeneration {
        self.generation
    }
    
    /// Упаковывает в u64 для компактного хранения
    pub fn to_u64(&self) -> u64 {
        ((self.generation as u64) << 32) | (self.index as u64)
    }
    
    /// Распаковывает из u64
    pub fn from_u64(value: u64) -> Self {
        Self {
            index: value as u32,
            generation: (value >> 32) as u32,
        }
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity({}v{})", self.index, self.generation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_pack_unpack() {
        let e = Entity::new(42, 7);
        let packed = e.to_u64();
        let unpacked = Entity::from_u64(packed);
        assert_eq!(e, unpacked);
    }
    
    #[test]
    fn test_entity_display() {
        let e = Entity::new(1, 3);
        assert_eq!(format!("{}", e), "Entity(1v3)");
    }
}
