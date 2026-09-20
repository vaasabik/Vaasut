//! Очередь событий для обработки в следующем кадре

use super::event::Event;
use std::collections::VecDeque;

/// Очередь событий — хранит события до их обработки
pub struct EventQueue {
    /// Очередь событий для обработки
    pending: VecDeque<Box<dyn Event>>,
    /// Максимальный размер очереди (защита от переполнения)
    max_size: usize,
}

impl EventQueue {
    /// Создаёт новую очередь событий
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            max_size: 10000,
        }
    }
    
    /// Создаёт очередь с максимальным размером
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            pending: VecDeque::new(),
            max_size,
        }
    }
    
    /// Добавляет событие в очередь
    pub fn push(&mut self, event: Box<dyn Event>) {
        if self.pending.len() < self.max_size {
            self.pending.push_back(event);
        }
    }
    
    /// Добавляет событие (удобная обёртка)
    pub fn emit(&mut self, event: impl Event + 'static) {
        self.push(Box::new(event));
    }
    
    /// Извлекает все события для обработки
    pub fn drain(&mut self) -> Vec<Box<dyn Event>> {
        self.pending.drain(..).collect()
    }
    
    /// Проверяет, есть ли события в очереди
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    
    /// Количество событий в очереди
    pub fn len(&self) -> usize {
        self.pending.len()
    }
    
    /// Очищает очередь
    pub fn clear(&mut self) {
        self.pending.clear();
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::event::SimpleEvent;
    
    #[test]
    fn test_event_queue() {
        let mut queue = EventQueue::new();
        
        queue.emit(SimpleEvent::new("event1"));
        queue.emit(SimpleEvent::new("event2"));
        
        assert_eq!(queue.len(), 2);
        assert!(!queue.is_empty());
        
        let events = queue.drain();
        assert_eq!(events.len(), 2);
        assert!(queue.is_empty());
    }
    
    #[test]
    fn test_event_queue_max_size() {
        let mut queue = EventQueue::with_max_size(2);
        
        queue.emit(SimpleEvent::new("event1"));
        queue.emit(SimpleEvent::new("event2"));
        queue.emit(SimpleEvent::new("event3")); // Не поместится
        
        assert_eq!(queue.len(), 2);
    }
}
