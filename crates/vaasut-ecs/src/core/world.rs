//! Мир — главный контейнер для всех сущностей и компонентов

use super::entity::EntityPool;
use crate::storage::ComponentStore;
use vaasut_core::Entity;

/// Мир — хранит все сущности и их компоненты
pub struct World {
    /// Пул сущностей
    entities: EntityPool,
    /// Хранилище компонентов
    components: ComponentStore,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: EntityPool::new(),
            components: ComponentStore::new(),
        }
    }
    
    /// Создаёт новую сущность
    pub fn spawn(&mut self) -> Entity {
        self.entities.spawn()
    }
    
    /// Удаляет сущность и все её компоненты
    pub fn despawn(&mut self, entity: Entity) -> bool {
        if self.entities.despawn(entity) {
            self.components.remove_all(entity);
            true
        } else {
            false
        }
    }
    
    /// Проверяет, жива ли сущность
    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities.is_alive(entity)
    }
    
    /// Количество живых сущностей
    pub fn entity_count(&self) -> usize {
        self.entities.alive_count()
    }
    
    /// Добавляет компонент сущности
    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        self.components.insert(entity, component);
    }
    
    /// Получает компонент сущности
    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.components.get(entity)
    }
    
    /// Получает изменяемый компонент сущности
    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.components.get_mut(entity)
    }
    
    /// Удаляет компонент у сущности
    pub fn remove_component<T: 'static>(&mut self, entity: Entity) -> Option<T> {
        self.components.remove(entity)
    }
    
    /// Проверяет, есть ли у сущности компонент
    pub fn has_component<T: 'static>(&self, entity: Entity) -> bool {
        self.components.contains::<T>(entity)
    }
    
    /// Возвращает все сущности с определённым компонентом
    pub fn query<T: 'static>(&self) -> Vec<Entity> {
        self.components.entities_with_component::<T>()
    }
    
    /// Возвращает все сущности с двумя компонентами
    pub fn query2<T1: 'static, T2: 'static>(&self) -> Vec<Entity> {
        let mut result = Vec::new();
        for entity in self.query::<T1>() {
            if self.has_component::<T2>(entity) {
                result.push(entity);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestComponent {
        value: i32,
    }
    
    #[test]
    fn test_world_spawn_despawn() {
        let mut world = World::new();
        
        let entity = world.spawn();
        assert!(world.is_alive(entity));
        
        world.despawn(entity);
        assert!(!world.is_alive(entity));
    }
    
    #[test]
    fn test_world_add_get_component() {
        let mut world = World::new();
        let entity = world.spawn();
        
        world.add_component(entity, TestComponent { value: 42 });
        
        let comp = world.get_component::<TestComponent>(entity);
        assert!(comp.is_some());
        assert_eq!(comp.unwrap().value, 42);
    }
    
    #[test]
    fn test_world_query() {
        let mut world = World::new();
        
        let e1 = world.spawn();
        world.add_component(e1, TestComponent { value: 1 });
        
        let e2 = world.spawn();
        world.add_component(e2, TestComponent { value: 2 });
        
        let e3 = world.spawn(); // Без компонента
        
        let entities = world.query::<TestComponent>();
        assert_eq!(entities.len(), 2);
    }
}
