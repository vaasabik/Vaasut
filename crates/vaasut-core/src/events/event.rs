//! Базовые типы событий

use std::any::Any;

/// Уникальный идентификатор типа события
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId(pub u64);

/// Трейт события — всё, что может быть событием
pub trait Event: Any + Send {
    /// Название события (для отладки)
    fn name(&self) -> &str;
    
    /// Возвращает себя как Any (для динамической диспетчеризации)
    fn as_any(&self) -> &dyn Any;
}

/// Обработчик событий
pub trait EventHandler: Send {
    /// Вызывается при получении события
    fn handle(&mut self, event: &dyn Event);
    
    /// Название обработчика (для отладки)
    fn name(&self) -> &str {
        "Unnamed Handler"
    }
}

/// Простое событие с именем
#[derive(Debug, Clone)]
pub struct SimpleEvent {
    name: String,
}

impl SimpleEvent {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl Event for SimpleEvent {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие с данными
#[derive(Debug, Clone)]
pub struct DataEvent<T: Clone + Send + 'static> {
    name: String,
    data: T,
}

impl<T: Clone + Send + 'static> DataEvent<T> {
    pub fn new(name: &str, data: T) -> Self {
        Self {
            name: name.to_string(),
            data,
        }
    }
    
    pub fn data(&self) -> &T {
        &self.data
    }
    
    pub fn into_data(self) -> T {
        self.data
    }
}

impl<T: Clone + Send + 'static> Event for DataEvent<T> {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Событие с колбэком-обработчиком
pub struct CallbackHandler {
    name: String,
    callback: Box<dyn FnMut(&dyn Event) + Send>,
}

impl CallbackHandler {
    pub fn new(name: &str, callback: impl FnMut(&dyn Event) + Send + 'static) -> Self {
        Self {
            name: name.to_string(),
            callback: Box::new(callback),
        }
    }
}

impl EventHandler for CallbackHandler {
    fn handle(&mut self, event: &dyn Event) {
        (self.callback)(event);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_event() {
        let event = SimpleEvent::new("test_event");
        assert_eq!(event.name(), "test_event");
    }
    
    #[test]
    fn test_data_event() {
        let event = DataEvent::new("player_health", 100);
        assert_eq!(event.name(), "player_health");
        assert_eq!(*event.data(), 100);
    }
    
    #[test]
    fn test_callback_handler() {
        let mut handled = false;
        
        let mut handler = CallbackHandler::new("test", |_event| {
            handled = true;
        });
        
        let event = SimpleEvent::new("test");
        handler.handle(&event);
        
        assert!(handled);
    }
}
