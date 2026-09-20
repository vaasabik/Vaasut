//! Игровые состояния (меню, игра, пауза и т.д.)

use vaasut_core::{StateMachine, State, StateContext};

/// Стандартные игровые состояния
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStateType {
    /// Загрузка
    Loading,
    /// Главное меню
    MainMenu,
    /// Игра
    Playing,
    /// Пауза
    Paused,
    /// Игра окончена
    GameOver,
}

impl GameStateType {
    pub fn name(&self) -> &'static str {
        match self {
            GameStateType::Loading => "loading",
            GameStateType::MainMenu => "main_menu",
            GameStateType::Playing => "playing",
            GameStateType::Paused => "paused",
            GameStateType::GameOver => "game_over",
        }
    }
    
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "loading" => Some(GameStateType::Loading),
            "main_menu" => Some(GameStateType::MainMenu),
            "playing" => Some(GameStateType::Playing),
            "paused" => Some(GameStateType::Paused),
            "game_over" => Some(GameStateType::GameOver),
            _ => None,
        }
    }
}

/// Состояние игры
pub struct GameState {
    game_type: GameStateType,
    on_enter: Option<Box<dyn FnMut()>>,
    on_update: Option<Box<dyn FnMut(f32)>>,
    on_exit: Option<Box<dyn FnMut()>>,
}

impl GameState {
    pub fn new(game_type: GameStateType) -> Self {
        Self {
            game_type,
            on_enter: None,
            on_update: None,
            on_exit: None,
        }
    }
    
    pub fn with_enter(mut self, callback: impl FnMut() + 'static) -> Self {
        self.on_enter = Some(Box::new(callback));
        self
    }
    
    pub fn with_update(mut self, callback: impl FnMut(f32) + 'static) -> Self {
        self.on_update = Some(Box::new(callback));
        self
    }
    
    pub fn with_exit(mut self, callback: impl FnMut() + 'static) -> Self {
        self.on_exit = Some(Box::new(callback));
        self
    }
}

impl State for GameState {
    fn name(&self) -> &str {
        self.game_type.name()
    }
    
    fn on_enter(&mut self, _ctx: &StateContext) {
        if let Some(callback) = &mut self.on_enter {
            callback();
        }
    }
    
    fn on_update(&mut self, ctx: &StateContext) {
        if let Some(callback) = &mut self.on_update {
            callback(ctx.delta_time);
        }
    }
    
    fn on_exit(&mut self, _ctx: &StateContext) {
        if let Some(callback) = &mut self.on_exit {
            callback();
        }
    }
}

/// Машина состояний игры
pub struct GameStateMachine {
    /// Внутренняя машина состояний
    machine: StateMachine,
}

impl GameStateMachine {
    /// Создаёт машину с стандартными состояниями
    pub fn new() -> Self {
        let mut machine = StateMachine::new();
        
        machine.add_state(GameState::new(GameStateType::Loading));
        machine.add_state(GameState::new(GameStateType::MainMenu));
        machine.add_state(GameState::new(GameStateType::Playing));
        machine.add_state(GameState::new(GameStateType::Paused));
        machine.add_state(GameState::new(GameStateType::GameOver));
        
        Self { machine }
    }
    
    /// Запускает с меню
    pub fn start_at_menu(&mut self) {
        self.machine.set_initial_state(GameStateType::MainMenu.name());
    }
    
    /// Запускает с игры
    pub fn start_playing(&mut self) {
        self.machine.set_initial_state(GameStateType::Playing.name());
    }
    
    /// Переход в меню
    pub fn go_to_menu(&mut self) {
        self.machine.transition_to(GameStateType::MainMenu.name());
    }
    
    /// Начинает игру
    pub fn start_game(&mut self) {
        self.machine.transition_to(GameStateType::Playing.name());
    }
    
    /// Пауза
    pub fn pause(&mut self) {
        if self.machine.is_in_state(GameStateType::Playing.name()) {
            self.machine.transition_to(GameStateType::Paused.name());
        }
    }
    
    /// Снять паузу
    pub fn resume(&mut self) {
        if self.machine.is_in_state(GameStateType::Paused.name()) {
            self.machine.transition_to(GameStateType::Playing.name());
        }
    }
    
    /// Игра окончена
    pub fn game_over(&mut self) {
        self.machine.transition_to(GameStateType::GameOver.name());
    }
    
    /// Обновление
    pub fn update(&mut self, delta_time: f32) {
        self.machine.update(delta_time);
    }
    
    /// Текущее состояние
    pub fn current_state(&self) -> Option<GameStateType> {
        self.machine.current_state()
            .and_then(GameStateType::from_name)
    }
    
    /// Находимся ли в меню?
    pub fn is_in_menu(&self) -> bool {
        self.machine.is_in_state(GameStateType::MainMenu.name())
    }
    
    /// Играем?
    pub fn is_playing(&self) -> bool {
        self.machine.is_in_state(GameStateType::Playing.name())
    }
    
    /// На паузе?
    pub fn is_paused(&self) -> bool {
        self.machine.is_in_state(GameStateType::Paused.name())
    }
}

impl Default for GameStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
