use super::entity::Entity;

/// Мир, хранящий все сущности
pub struct World {
    next_id: u64,
}

impl World {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }
    
    pub fn spawn(&mut self) -> Entity {
        let entity = Entity(self.next_id);
        self.next_id += 1;
        entity
    }
}
