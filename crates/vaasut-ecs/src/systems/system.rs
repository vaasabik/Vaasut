/// Система, обрабатывающая сущности
pub trait System {
    fn update(&mut self, world: &mut super::super::core::world::World);
}
