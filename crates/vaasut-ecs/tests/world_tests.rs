use vaasut_ecs::{World, Visible, Transform2DComponent, Sprite, TextureId, Camera2D};

#[test]
fn test_create_world() {
    let world = World::new();
    assert_eq!(world.entity_count(), 0);
}

#[test]
fn test_spawn_entity() {
    let mut world = World::new();
    let entity = world.spawn();
    assert!(world.is_alive(entity));
    assert_eq!(world.entity_count(), 1);
}

#[test]
fn test_add_components() {
    let mut world = World::new();
    let entity = world.spawn();
    
    world.add_component(entity, Visible::shown());
    world.add_component(entity, Transform2DComponent::from_position(10.0, 20.0));
    
    assert!(world.has_component::<Visible>(entity));
    assert!(world.has_component::<Transform2DComponent>(entity));
    
    let transform = world.get_component::<Transform2DComponent>(entity).unwrap();
    assert_eq!(transform.position().x, 10.0);
    assert_eq!(transform.position().y, 20.0);
}

#[test]
fn test_query_entities() {
    let mut world = World::new();
    
    // Создаём 3 спрайта
    for i in 0..3 {
        let entity = world.spawn();
        world.add_component(entity, Transform2DComponent::from_position(i as f32, 0.0));
        world.add_component(entity, Sprite::new(TextureId(i as u64)));
    }
    
    // Создаём 2 объекта без спрайтов
    for _ in 0..2 {
        world.spawn();
    }
    
    let sprites = world.query::<Sprite>();
    assert_eq!(sprites.len(), 3);
}

#[test]
fn test_despawn_removes_components() {
    let mut world = World::new();
    let entity = world.spawn();
    
    world.add_component(entity, Visible::shown());
    world.add_component(entity, Transform2DComponent::default());
    
    world.despawn(entity);
    
    assert!(!world.is_alive(entity));
    assert!(!world.has_component::<Visible>(entity));
}
