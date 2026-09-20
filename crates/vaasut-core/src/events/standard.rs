//! Стандартные события движка

use super::event::{Event, DataEvent};
use std::any::Any;

/// Событие изменения состояния машины состояний
#[derive(Debug, Clone)]
pub struct StateChangedEvent {
    /// Имя предыдущего состояния
    pub previous_state: String,
    /// Имя нового состояния
    pub new_state: String,
}

impl StateChangedEvent {
    pub fn new(previous: &str, new: &str) -> Self {
        Self {
            previous_state: previous.to_string(),
            new_state: new.to_string(),
        }
    }
}

impl Event for StateChangedEvent {
    fn name(&self) -> &str {
        "state_changed"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие создания сущности
#[derive(Debug, Clone)]
pub struct EntityCreatedEvent {
    pub entity_id: u64,
}

impl Event for EntityCreatedEvent {
    fn name(&self) -> &str {
        "entity_created"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие удаления сущности
#[derive(Debug, Clone)]
pub struct EntityDestroyedEvent {
    pub entity_id: u64,
}

impl Event for EntityDestroyedEvent {
    fn name(&self) -> &str {
        "entity_destroyed"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие выбора объекта (для редактора)
#[derive(Debug, Clone)]
pub struct ObjectSelectedEvent {
    pub entity_id: u64,
}

impl Event for ObjectSelectedEvent {
    fn name(&self) -> &str {
        "object_selected"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие изменения свойства объекта (для редактора)
#[derive(Debug, Clone)]
pub struct PropertyChangedEvent {
    pub entity_id: u64,
    pub property_name: String,
}

impl Event for PropertyChangedEvent {
    fn name(&self) -> &str {
        "property_changed"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие начала воспроизведения (для редактора)
pub struct PlayStartedEvent;

impl Event for PlayStartedEvent {
    fn name(&self) -> &str {
        "play_started"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие остановки воспроизведения (для редактора)
pub struct PlayStoppedEvent;

impl Event for PlayStoppedEvent {
    fn name(&self) -> &str {
        "play_stopped"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие паузы
pub struct PausedEvent;

impl Event for PausedEvent {
    fn name(&self) -> &str {
        "paused"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие продолжения
pub struct ResumedEvent;

impl Event for ResumedEvent {
    fn name(&self) -> &str {
        "resumed"
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}
