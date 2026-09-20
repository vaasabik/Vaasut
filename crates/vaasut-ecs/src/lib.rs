//! Vaasut ECS: Простая система Entity-Component-System

use std::collections::HashMap;
use vaasut_core::Entity;

/// Мир, хранящий все сущности и их компоненты
pub struct World {
    entities: Vec<Entity>,
    next_id: u64,
    // В будущем здесь будут Sparse Sets для компонентов
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 0,
        }
    }

    /// Создает новую сущность
    pub fn spawn(&mut self) -> Entity {
        let entity = Entity::new(self.next_id);
        self.next_id += 1;
        self.entities.push(entity);
        entity
    }

    /// Удаляет сущность
    pub fn despawn(&mut self, entity: Entity) {
        self.entities.retain(|&e| e != entity);
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}
