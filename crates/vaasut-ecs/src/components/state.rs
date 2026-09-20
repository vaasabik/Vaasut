//! Компонент машины состояний для объектов

use vaasut_core::{StateMachine, State, StateContext};
use vaasut_core::state_machine::state::CallbackState;

/// Компонент машины состояний для сущности
/// 
/// Позволяет объектам иметь свои состояния (например, враг: патрулирует -> преследует -> атакует)
pub struct StateMachineComponent {
    /// Внутренняя машина состояний
    pub machine: StateMachine,
    /// Активна ли машина
    pub enabled: bool,
}

impl StateMachineComponent {
    pub fn new() -> Self {
        Self {
            machine: StateMachine::new(),
            enabled: true,
        }
    }
    
    /// Добавляет состояние
    pub fn add_state(&mut self, state: impl State + 'static) {
        self.machine.add_state(state);
    }
    
    /// Добавляет состояние с колбэками (быстрый способ)
    pub fn add_callback_state(&mut self, name: &str) -> &mut Self {
        self.machine.add_state(CallbackState::new(name));
        self
    }
    
    /// Устанавливает начальное состояние
    pub fn set_initial(&mut self, state_name: &str) {
        self.machine.set_initial_state(state_name);
    }
    
    /// Обновление
    pub fn update(&mut self, delta_time: f32) {
        if self.enabled {
            self.machine.update(delta_time);
        }
    }
    
    /// Текущее состояние
    pub fn current_state(&self) -> Option<&str> {
        self.machine.current_state()
    }
    
    /// Переход в состояние
    pub fn transition_to(&mut self, state_name: &str) -> bool {
        self.machine.transition_to(state_name)
    }
    
    /// Проверка состояния
    pub fn is_in_state(&self, state_name: &str) -> bool {
        self.machine.is_in_state(state_name)
    }
}

impl Default for StateMachineComponent {
    fn default() -> Self {
        Self::new()
    }
}

/// Система для обновления всех машин состояний
pub struct StateMachineSystem;

impl crate::systems::System for StateMachineSystem {
    fn update(&mut self, world: &mut crate::core::World, delta_time: f32) {
        let entities: Vec<_> = world.query::<StateMachineComponent>()
            .into_iter()
            .collect();
        
        for entity in entities {
            if let Some(component) = world.get_component_mut::<StateMachineComponent>(entity) {
                if component.enabled {
                    component.machine.update(delta_time);
                }
            }
        }
    }
    
    fn name(&self) -> &str {
        "StateMachineSystem"
    }
}
