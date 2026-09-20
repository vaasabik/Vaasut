//! Vaasut Runtime: Ядро игрового процесса

pub mod loop_control;
pub mod states;
pub mod state_machine;

pub use state_machine::{GameState, GameStateMachine};
