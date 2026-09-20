//! Машина состояний — фундаментальный паттерн для управления режимами работы
//! 
//! Используется на нескольких уровнях:
//! - Редактор: режимы редактирования (Edit/Play/Pause)
//! - Рантайм: игровые состояния (Menu/Playing/Paused/GameOver)
//! - Объекты: состояния персонажей (Idle/Run/Jump/Attack)
//! - Анимации: переходы между анимациями

pub mod state;
pub mod machine;
pub mod transition;

pub use state::{State, StateContext};
pub use machine::StateMachine;
pub use transition::Transition;
