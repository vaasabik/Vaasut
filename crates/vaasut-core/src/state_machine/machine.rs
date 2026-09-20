//! Машина состояний — управляет переключением между состояниями

use std::collections::HashMap;
use super::state::{State, StateContext};
use super::transition::Transition;
use crate::events::{EventBus, StateChangedEvent};

/// Машина состояний — универсальная реализация
pub struct StateMachine {
    /// Состояния (имя -> состояние)
    states: HashMap<String, Box<dyn State>>,
    /// Переходы
    transitions: Vec<Transition>,
    /// Текущее состояние
    current_state: Option<String>,
    /// Контекст (время и т.д.)
    context: StateContext,
    /// История состояний (для отладки)
    history: Vec<String>,
    /// Максимальный размер истории
    max_history: usize,
    /// Шина событий (опционально)
    event_bus: Option<EventBus>,
}

impl StateMachine {
    /// Создаёт пустую машину состояний
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            transitions: Vec::new(),
            current_state: None,
            context: StateContext::default(),
            history: Vec::new(),
            max_history: 100,
            event_bus: None,
        }
    }
    
    /// Создаёт машину с шиной событий
    pub fn with_event_bus(event_bus: EventBus) -> Self {
        let mut machine = Self::new();
        machine.event_bus = Some(event_bus);
        machine
    }
    
    /// Устанавливает шину событий
    pub fn set_event_bus(&mut self, event_bus: EventBus) {
        self.event_bus = Some(event_bus);
    }
    
    /// Добавляет состояние
    pub fn add_state(&mut self, state: impl State + 'static) {
        let name = state.name().to_string();
        self.states.insert(name, Box::new(state));
    }
    
    /// Добавляет переход
    pub fn add_transition(&mut self, transition: Transition) {
        self.transitions.push(transition);
    }
    
    /// Устанавливает начальное состояние
    pub fn set_initial_state(&mut self, state_name: &str) -> bool {
        if self.states.contains_key(state_name) {
            self.current_state = Some(state_name.to_string());
            self.add_to_history(state_name);
            
            if let Some(state) = self.states.get_mut(state_name) {
                state.on_enter(&self.context);
            }
            
            // Отправляем событие
            self.emit_state_changed("", state_name);
            
            true
        } else {
            false
        }
    }
    
    /// Переключается на состояние
    pub fn transition_to(&mut self, state_name: &str) -> bool {
        if !self.states.contains_key(state_name) {
            return false;
        }
        
        let previous = self.current_state.clone();
        
        // Выходим из текущего состояния
        if let Some(current) = &self.current_state {
            if let Some(state) = self.states.get_mut(current) {
                state.on_exit(&self.context);
            }
        }
        
        // Переключаемся
        self.current_state = Some(state_name.to_string());
        self.context.time_in_state = 0.0;
        self.add_to_history(state_name);
        
        // Входим в новое состояние
        if let Some(state) = self.states.get_mut(state_name) {
            state.on_enter(&self.context);
        }
        
        // Отправляем событие
        let prev_name = previous.as_deref().unwrap_or("");
        self.emit_state_changed(prev_name, state_name);
        
        true
    }
    
    /// Обновляет машину состояний (вызывать каждый кадр)
    pub fn update(&mut self, delta_time: f32) {
        // Обновляем время
        self.context.delta_time = delta_time;
        self.context.total_time += delta_time;
        self.context.time_in_state += delta_time;
        
        // Проверяем переходы
        if let Some(current) = self.current_state.clone() {
            let mut applicable_transitions: Vec<usize> = Vec::new();
            
            // Собираем применимые переходы
            for (i, transition) in self.transitions.iter_mut().enumerate() {
                if transition.is_applicable(&current) && transition.check() {
                    applicable_transitions.push(i);
                }
            }
            
            // Выбираем переход с наибольшим приоритетом
            if !applicable_transitions.is_empty() {
                let best_transition = applicable_transitions
                    .iter()
                    .max_by_key(|&&i| self.transitions[i].priority)
                    .map(|&i| self.transitions[i].to.clone());
                
                if let Some(target) = best_transition {
                    self.transition_to(&target);
                }
            }
        }
        
        // Обновляем текущее состояние
        if let Some(current) = &self.current_state {
            if let Some(state) = self.states.get_mut(current) {
                state.on_update(&self.context);
            }
        }
    }
    
    /// Возвращает текущее состояние
    pub fn current_state(&self) -> Option<&str> {
        self.current_state.as_deref()
    }
    
    /// Проверяет, находимся ли мы в определённом состоянии
    pub fn is_in_state(&self, state_name: &str) -> bool {
        self.current_state.as_deref() == Some(state_name)
    }
    
    /// Возвращает время в текущем состоянии
    pub fn time_in_current_state(&self) -> f32 {
        self.context.time_in_state
    }
    
    /// Возвращает историю состояний
    pub fn history(&self) -> &[String] {
        &self.history
    }
    
    /// Добавляет в историю
    fn add_to_history(&mut self, state_name: &str) {
        self.history.push(state_name.to_string());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }
    
    /// Отправляет событие изменения состояния
    fn emit_state_changed(&mut self, previous: &str, new: &str) {
        if let Some(bus) = &mut self.event_bus {
            bus.emit(StateChangedEvent::new(previous, new));
            bus.process(); // Обрабатываем сразу
        }
    }
    
    /// Возвращает количество состояний
    pub fn state_count(&self) -> usize {
        self.states.len()
    }
    
    /// Возвращает список всех состояний
    pub fn state_names(&self) -> Vec<&str> {
        self.states.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_machine::state::SimpleState;
    use crate::events::CallbackHandler;
    use std::sync::{Arc, Mutex};
    
    #[test]
    fn test_state_machine_with_events() {
        let mut event_bus = EventBus::new();
        let state_changes = Arc::new(Mutex::new(0));
        
        let counter = state_changes.clone();
        event_bus.subscribe("state_changed", Box::new(CallbackHandler::new("counter", move |_event| {
            *counter.lock().unwrap() += 1;
        })));
        
        let mut machine = StateMachine::with_event_bus(event_bus);
        machine.add_state(SimpleState::new("idle"));
        machine.add_state(SimpleState::new("running"));
        
        machine.set_initial_state("idle");
        machine.transition_to("running");
        
        assert_eq!(*state_changes.lock().unwrap(), 2);
    }
}
