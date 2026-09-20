//! Состояния и их жизненный цикл

/// Контекст для состояния — данные, доступные при обновлении
#[derive(Debug, Clone, Default)]
pub struct StateContext {
    /// Время в текущем состоянии (в секундах)
    pub time_in_state: f32,
    /// Общее время работы
    pub total_time: f32,
    /// Delta time (время последнего кадра)
    pub delta_time: f32,
}

/// Трейт состояния — всё, что может быть состоянием
pub trait State {
    /// Название состояния (для отладки и логов)
    fn name(&self) -> &str;
    
    /// Вызывается при входе в состояние
    fn on_enter(&mut self, _ctx: &StateContext) {}
    
    /// Вызывается каждый кадр, пока состояние активно
    fn on_update(&mut self, _ctx: &StateContext) {}
    
    /// Вызывается при выходе из состояния
    fn on_exit(&mut self, _ctx: &StateContext) {}
}

/// Простое состояние без логики (только имя)
#[derive(Debug, Clone)]
pub struct SimpleState {
    name: String,
}

impl SimpleState {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl State for SimpleState {
    fn name(&self) -> &str {
        &self.name
    }
}

/// Состояние с колбэками (для быстрого создания)
pub struct CallbackState {
    name: String,
    on_enter: Option<Box<dyn FnMut(&StateContext)>>,
    on_update: Option<Box<dyn FnMut(&StateContext)>>,
    on_exit: Option<Box<dyn FnMut(&StateContext)>>,
}

impl CallbackState {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            on_enter: None,
            on_update: None,
            on_exit: None,
        }
    }
    
    pub fn with_enter(mut self, callback: impl FnMut(&StateContext) + 'static) -> Self {
        self.on_enter = Some(Box::new(callback));
        self
    }
    
    pub fn with_update(mut self, callback: impl FnMut(&StateContext) + 'static) -> Self {
        self.on_update = Some(Box::new(callback));
        self
    }
    
    pub fn with_exit(mut self, callback: impl FnMut(&StateContext) + 'static) -> Self {
        self.on_exit = Some(Box::new(callback));
        self
    }
}

impl State for CallbackState {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn on_enter(&mut self, ctx: &StateContext) {
        if let Some(callback) = &mut self.on_enter {
            callback(ctx);
        }
    }
    
    fn on_update(&mut self, ctx: &StateContext) {
        if let Some(callback) = &mut self.on_update {
            callback(ctx);
        }
    }
    
    fn on_exit(&mut self, ctx: &StateContext) {
        if let Some(callback) = &mut self.on_exit {
            callback(ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_state() {
        let state = SimpleState::new("idle");
        assert_eq!(state.name(), "idle");
    }
    
    #[test]
    fn test_callback_state() {
        let mut entered = false;
        
        let mut state = CallbackState::new("test")
            .with_enter(|_ctx| {
                entered = true;
            });
        
        let ctx = StateContext::default();
        state.on_enter(&ctx);
        
        assert!(entered);
    }
}
