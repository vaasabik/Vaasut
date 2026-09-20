//! Система событий — механизм общения между компонентами движка

pub mod event;
pub mod queue;
pub mod bus;
pub mod standard;

pub use event::{Event, EventHandler, EventId, CallbackHandler, SimpleEvent, DataEvent};
pub use queue::EventQueue;
pub use bus::EventBus;
pub use standard::*;
