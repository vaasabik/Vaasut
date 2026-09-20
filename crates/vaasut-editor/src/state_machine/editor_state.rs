//! Режимы работы редактора

use vaasut_core::{StateMachine, State, StateContext};

/// Режимы редактора
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    /// Режим редактирования (можно двигать объекты, гизмо активны)
    Edit,
    /// Режим игры (рантайм запущен, можно тестировать)
    Play,
    /// Пауза (рантайм остановлен, можно инспектировать)
    Pause,
    /// Загрузка
    Loading,
}

impl EditorMode {
    pub fn name(&self) -> &'static str {
        match self {
            EditorMode::Edit => "edit",
            EditorMode::Play => "play",
            EditorMode::Pause => "pause",
            EditorMode::Loading => "loading",
        }
    }
    
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "edit" => Some(EditorMode::Edit),
            "play" => Some(EditorMode::Play),
            "pause" => Some(EditorMode::Pause),
            "loading" => Some(EditorMode::Loading),
            _ => None,
        }
    }
    
    /// Можно ли редактировать в этом режиме?
    pub fn can_edit(&self) -> bool {
        matches!(self, EditorMode::Edit)
    }
    
    /// Запущен ли рантайм?
    pub fn is_runtime_active(&self) -> bool {
        matches!(self, EditorMode::Play | EditorMode::Pause)
    }
}

/// Состояние редактора
pub struct EditorState {
    mode: EditorMode,
}

impl EditorState {
    pub fn new(mode: EditorMode) -> Self {
        Self { mode }
    }
}

impl State for EditorState {
    fn name(&self) -> &str {
        self.mode.name()
    }
}

/// Машина состояний редактора
pub struct EditorStateMachine {
    machine: StateMachine,
}

impl EditorStateMachine {
    pub fn new() -> Self {
        let mut machine = StateMachine::new();
        
        machine.add_state(EditorState::new(EditorMode::Edit));
        machine.add_state(EditorState::new(EditorMode::Play));
        machine.add_state(EditorState::new(EditorMode::Pause));
        machine.add_state(EditorState::new(EditorMode::Loading));
        
        Self { machine }
    }
    
    /// Запуск в режиме редактирования
    pub fn start_in_edit_mode(&mut self) {
        self.machine.set_initial_state(EditorMode::Edit.name());
    }
    
    /// Начать игру (перейти в Play)
    pub fn play(&mut self) {
        if self.machine.is_in_state(EditorMode::Edit.name()) {
            self.machine.transition_to(EditorMode::Play.name());
        }
    }
    
    /// Пауза
    pub fn pause(&mut self) {
        if self.machine.is_in_state(EditorMode::Play.name()) {
            self.machine.transition_to(EditorMode::Pause.name());
        }
    }
    
    /// Продолжить
    pub fn resume(&mut self) {
        if self.machine.is_in_state(EditorMode::Pause.name()) {
            self.machine.transition_to(EditorMode::Play.name());
        }
    }
    
    /// Остановить (вернуться в Edit)
    pub fn stop(&mut self) {
        self.machine.transition_to(EditorMode::Edit.name());
    }
    
    /// Обновление
    pub fn update(&mut self, delta_time: f32) {
        self.machine.update(delta_time);
    }
    
    /// Текущий режим
    pub fn current_mode(&self) -> Option<EditorMode> {
        self.machine.current_state()
            .and_then(EditorMode::from_name)
    }
    
    /// В режиме редактирования?
    pub fn is_editing(&self) -> bool {
        self.machine.is_in_state(EditorMode::Edit.name())
    }
    
    /// Играем?
    pub fn is_playing(&self) -> bool {
        self.machine.is_in_state(EditorMode::Play.name())
    }
    
    /// На паузе?
    pub fn is_paused(&self) -> bool {
        self.machine.is_in_state(EditorMode::Pause.name())
    }
    
    /// Рантайм активен?
    pub fn is_runtime_active(&self) -> bool {
        self.current_mode()
            .map(|m| m.is_runtime_active())
            .unwrap_or(false)
    }
}

impl Default for EditorStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
