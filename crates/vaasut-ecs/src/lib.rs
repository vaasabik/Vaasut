//! Vaasut ECS: Система сущностей и компонентов

pub mod core;
pub mod components;
pub mod storage;
pub mod systems;

// Реэкспортируем основные типы
pub use core::{World, EntityPool};
pub use systems::{System, SystemScheduler};
pub use components::*;

/// Предопределённые "бандлы" компонентов для быстрого создания объектов
pub mod bundles {
    use super::*;
    use vaasut_math::Color;
    use vaasut_core::Entity;
    
    /// Создаёт 2D спрайт с трансформом
    pub fn sprite_bundle(
        world: &mut World,
        texture: TextureId,
        x: f32,
        y: f32,
    ) -> Entity {
        let entity = world.spawn();
        world.add_component(entity, Transform2DComponent::from_position(x, y));
        world.add_component(entity, Sprite::new(texture));
        world.add_component(entity, Visible::shown());
        entity
    }
    
    /// Создаёт 3D объект с мешем и материалом
    pub fn mesh_bundle(
        world: &mut World,
        mesh: MeshId,
        material: MaterialId,
        x: f32,
        y: f32,
        z: f32,
    ) -> Entity {
        let entity = world.spawn();
        world.add_component(entity, Transform3DComponent::from_position(x, y, z));
        world.add_component(entity, MeshComponent::new(mesh, material));
        world.add_component(entity, Visible::shown());
        entity
    }
    
    /// Создаёт 2D камеру
    pub fn camera2d_bundle(world: &mut World) -> Entity {
        let entity = world.spawn();
        world.add_component(entity, Transform2DComponent::default());
        world.add_component(entity, Camera2D::new());
        world.add_component(entity, Visible::shown());
        entity
    }
    
    /// Создаёт 3D камеру
    pub fn camera3d_bundle(world: &mut World) -> Entity {
        let entity = world.spawn();
        world.add_component(entity, Transform3DComponent::default());
        world.add_component(entity, Camera3D::new());
        world.add_component(entity, Visible::shown());
        entity
    }
    
    /// Создаёт направленный свет (солнце)
    pub fn directional_light_bundle(
        world: &mut World,
        color: Color,
        intensity: f32,
    ) -> Entity {
        let entity = world.spawn();
        world.add_component(entity, Transform3DComponent::default());
        world.add_component(entity, Light::directional(color, intensity));
        entity
    }
    
    /// Создаёт точечный свет
    pub fn point_light_bundle(
        world: &mut World,
        color: Color,
        intensity: f32,
        range: f32,
        x: f32,
        y: f32,
        z: f32,
    ) -> Entity {
        let entity = world.spawn();
        world.add_component(entity, Transform3DComponent::from_position(x, y, z));
        world.add_component(entity, Light::point(color, intensity, range));
        entity
    }
    
    /// Создаёт объект с машиной состояний (для врагов, персонажей)
    pub fn stateful_entity_bundle(
        world: &mut World,
        x: f32,
        y: f32,
        z: f32,
    ) -> Entity {
        let entity = world.spawn();
        world.add_component(entity, Transform3DComponent::from_position(x, y, z));
        world.add_component(entity, StateMachineComponent::new());
        world.add_component(entity, Visible::shown());
        entity
    }
}
