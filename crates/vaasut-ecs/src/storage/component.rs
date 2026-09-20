//! Хранилище компонентов

use std::any::{Any, TypeId};
use std::collections::HashMap;
use vaasut_core::Entity;

/// Хранилище всех компонентов, сгруппированных по типу
pub struct ComponentStore {
    /// storage[TypeId] -> HashMap<Entity, Box<dyn Any>>
    storages: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
}

impl ComponentStore {
    pub fn new() -> Self {
        Self {
            storages: HashMap::new(),
        }
    }
    
    /// Добавляет компонент сущности
    pub fn insert<T: 'static>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        self.storages
            .entry(type_id)
            .or_insert_with(HashMap::new)
            .insert(entity, Box::new(component));
    }
    
    /// Получает ссылку на компонент
    pub fn get<T: 'static>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .get(&type_id)
            .and_then(|storage| storage.get(&entity))
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
    
    /// Получает изменяемую ссылку на компонент
    pub fn get_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .get_mut(&type_id)
            .and_then(|storage| storage.get_mut(&entity))
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }
    
    /// Удаляет компонент и возвращает его
    pub fn remove<T: 'static>(&mut self, entity: Entity) -> Option<T> {
        let type_id = TypeId::of::<T>();
        self.storages
            .get_mut(&type_id)
            .and_then(|storage| storage.remove(&entity))
            .and_then(|boxed| boxed.downcast::<T>().ok())
            .map(|boxed| *boxed)
    }
    
    /// Проверяет наличие компонента
    pub fn contains<T: 'static>(&self, entity: Entity) -> bool {
        let type_id = TypeId::of::<T>();
        self.storages
            .get(&type_id)
            .map_or(false, |storage| storage.contains_key(&entity))
    }
    
    /// Возвращает все сущности с данным компонентом
    pub fn entities_with_component<T: 'static>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<T>();
        self.storages
            .get(&type_id)
            .map(|storage| storage.keys().cloned().collect())
            .unwrap_or_default()
    }
    
    /// Удаляет все компоненты сущности
    pub fn remove_all(&mut self, entity: Entity) {
        for storage in self.storages.values_mut() {
            storage.remove(&entity);
        }
    }
}
