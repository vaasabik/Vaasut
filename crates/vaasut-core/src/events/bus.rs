//! Шина событий — центральный хаб для отправки и получения событий

use super::event::{Event, EventHandler};
use super::queue::EventQueue;
use std::collections::HashMap;

/// Шина событий — центральный хаб для всей системы
/// 
/// Все события проходят через шину. Компоненты могут:
/// - Отправлять события (`emit`)
/// - Подписываться на события (`subscribe`)
/// - Обрабатывать события в следующем кадре (`process`)
pub struct EventBus {
    /// Очередь событий
    queue: EventQueue,
    /// Обработчики событий (имя события -> список обработчиков)
    handlers: HashMap<String, Vec<Box<dyn EventHandler>>>,
    /// Глобальные обработчики (вызываются для всех событий)
    global_handlers: Vec<Box<dyn EventHandler>>,
    /// Счётчик обработанных событий (для отладки)
    processed_count: u64,
}

impl EventBus {
    /// Создаёт новую шину событий
    pub fn new() -> Self {
        Self {
            queue: EventQueue::new(),
            handlers: HashMap::new(),
            global_handlers: Vec::new(),
            processed_count: 0,
        }
    }
    
    /// Отправляет событие в очередь
    pub fn emit(&mut self, event: impl Event + 'static) {
        self.queue.emit(event);
    }
    
    /// Отправляет событие немедленно (без очереди)
    pub fn dispatch(&mut self, event: &dyn Event) {
        // Вызываем обработчики для этого типа события
        if let Some(handlers) = self.handlers.get_mut(event.name()) {
            for handler in handlers {
                handler.handle(event);
            }
        }
        
        // Вызываем глобальные обработчики
        for handler in &mut self.global_handlers {
            handler.handle(event);
        }
        
        self.processed_count += 1;
    }
    
    /// Подписывает обработчик на определённое событие
    pub fn subscribe(&mut self, event_name: &str, handler: Box<dyn EventHandler>) {
        self.handlers
            .entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(handler);
    }
    
    /// Подписывает обработчик на все события
    pub fn subscribe_global(&mut self, handler: Box<dyn EventHandler>) {
        self.global_handlers.push(handler);
    }
    
    /// Обрабатывает все события из очереди
    pub fn process(&mut self) {
        let events = self.queue.drain();
        
        for event in events {
            self.dispatch(&*event);
        }
    }
    
    /// Обновление шины (вызывать каждый кадр)
    pub fn update(&mut self) {
        self.process();
    }
    
    /// Количество обработанных событий
    pub fn processed_count(&self) -> u64 {
        self.processed_count
    }
    
    /// Количество событий в очереди
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
    
    /// Очищает очередь
    pub fn clear(&mut self) {
        self.queue.clear();
    }
    
    /// Проверяет, есть ли обработчики для события
    pub fn has_handlers(&self, event_name: &str) -> bool {
        self.handlers.contains_key(event_name)
    }
    
    /// Количество подписчиков на событие
    pub fn handler_count(&self, event_name: &str) -> usize {
        self.handlers.get(event_name).map(|h| h.len()).unwrap_or(0)
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::event::{SimpleEvent, CallbackHandler};
    use std::sync::{Arc, Mutex};
    
    #[test]
    fn test_event_bus_emit_process() {
        let mut bus = EventBus::new();
        let counter = Arc::new(Mutex::new(0));
        
        let counter_clone = counter.clone();
        bus.subscribe("test_event", Box::new(CallbackHandler::new("counter", move |_event| {
            *counter_clone.lock().unwrap() += 1;
        })));
        
        bus.emit(SimpleEvent::new("test_event"));
        
        assert_eq!(bus.pending_count(), 1);
        
        bus.process();
        
        assert_eq!(bus.pending_count(), 0);
        assert_eq!(*counter.lock().unwrap(), 1);
        assert_eq!(bus.processed_count(), 1);
    }
    
    #[test]
    fn test_event_bus_multiple_handlers() {
        let mut bus = EventBus::new();
        let counter = Arc::new(Mutex::new(0));
        
        // Подписываем два обработчика на одно событие
        for _ in 0..2 {
            let counter_clone = counter.clone();
            bus.subscribe("test_event", Box::new(CallbackHandler::new("counter", move |_event| {
                *counter_clone.lock().unwrap() += 1;
            })));
        }
        
        bus.emit(SimpleEvent::new("test_event"));
        bus.process();
        
        assert_eq!(*counter.lock().unwrap(), 2);
    }
    
    #[test]
    fn test_event_bus_global_handler() {
        let mut bus = EventBus::new();
        let counter = Arc::new(Mutex::new(0));
        
        let counter_clone = counter.clone();
        bus.subscribe_global(Box::new(CallbackHandler::new("global", move |_event| {
            *counter_clone.lock().unwrap() += 1;
        })));
        
        bus.emit(SimpleEvent::new("event1"));
        bus.emit(SimpleEvent::new("event2"));
        bus.process();
        
        // Глобальный обработчик должен получить оба события
        assert_eq!(*counter.lock().unwrap(), 2);
    }
}
