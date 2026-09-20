//! Управление сущностями

use vaasut_core::Entity;
use vaasut_core::entities::{EntityIndex, EntityGeneration};

/// Пул сущностей с переиспользованием индексов
pub struct EntityPool {
    /// Список свободных индексов для переиспользования
    free_indices: Vec<EntityIndex>,
    /// Поколения для каждого индекса
    generations: Vec<EntityGeneration>,
    /// Следующий индекс для новой сущности
    next_index: EntityIndex,
    /// Количество живых сущностей
    alive_count: usize,
}

impl EntityPool {
    pub fn new() -> Self {
        Self {
            free_indices: Vec::new(),
            generations: Vec::new(),
            next_index: 0,
            alive_count: 0,
        }
    }
    
    /// Создаёт новую сущность
    pub fn spawn(&mut self) -> Entity {
        self.alive_count += 1;
        
        if let Some(index) = self.free_indices.pop() {
            // Переиспользуем старый индекс с новым поколением
            let generation = self.generations[index as usize];
            Entity::new(index, generation)
        } else {
            // Создаём новый индекс
            let index = self.next_index;
            self.next_index += 1;
            self.generations.push(0);
            Entity::new(index, 0)
        }
    }
    
    /// Удаляет сущность
    pub fn despawn(&mut self, entity: Entity) -> bool {
        let index = entity.index as usize;
        
        if index >= self.generations.len() {
            return false;
        }
        
        if self.generations[index] != entity.generation {
            return false; // Устаревшая ссылка
        }
        
        // Увеличиваем поколение и добавляем индекс в свободные
        self.generations[index] += 1;
        self.free_indices.push(entity.index);
        self.alive_count -= 1;
        true
    }
    
    /// Проверяет, жива ли сущность
    pub fn is_alive(&self, entity: Entity) -> bool {
        let index = entity.index as usize;
        index < self.generations.len() && self.generations[index] == entity.generation
    }
    
    /// Количество живых сущностей
    pub fn alive_count(&self) -> usize {
        self.alive_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_entity_pool_spawn_despawn() {
        let mut pool = EntityPool::new();
        
        let e1 = pool.spawn();
        assert!(pool.is_alive(e1));
        assert_eq!(pool.alive_count(), 1);
        
        pool.despawn(e1);
        assert!(!pool.is_alive(e1));
        assert_eq!(pool.alive_count(), 0);
    }
    
    #[test]
    fn test_entity_pool_reuse_index() {
        let mut pool = EntityPool::new();
        
        let e1 = pool.spawn();
        let index1 = e1.index;
        
        pool.despawn(e1);
        
        let e2 = pool.spawn();
        assert_eq!(e2.index, index1); // Тот же индекс
        assert_eq!(e2.generation, 1); // Но новое поколение
        
        // Старая ссылка должна быть невалидной
        assert!(!pool.is_alive(e1));
        assert!(pool.is_alive(e2));
    }
}
