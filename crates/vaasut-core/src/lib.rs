//! Vaasut Core: Базовые типы и структуры для всего движка

pub mod entities;
pub mod errors;
pub mod events;
pub mod logging;
pub mod primitives;
pub mod state_machine;

pub use entities::Entity;
pub use primitives::{Name, Dimension};
pub use errors::VaasutError;
pub use state_machine::{StateMachine, State, Transition};
pub use state_machine::state::StateContext;
pub use events::{Event, EventBus, EventQueue, EventHandler};
