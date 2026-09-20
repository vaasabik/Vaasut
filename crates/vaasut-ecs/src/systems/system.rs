//! Системы — логика, обрабатывающая сущности

use crate::core::world::World;

/// Трейт системы — логика, которая выполняется каждый кадр
pub trait System {
    /// Обновление системы
    fn update(&mut self, world: &mut World, delta_time: f32);
    
    /// Имя системы (для отладки)
    fn name(&self) -> &str {
        "Unnamed System"
    }
}

/// Планировщик систем — управляет порядком выполнения
pub struct SystemScheduler {
    systems: Vec<Box<dyn System>>,
}

impl SystemScheduler {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }
    
    /// Добавляет систему
    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }
    
    /// Обновляет все системы
    pub fn update(&mut self, world: &mut World, delta_time: f32) {
        for system in &mut self.systems {
            system.update(world, delta_time);
        }
    }
}
