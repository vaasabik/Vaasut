//! Vaasut Runtime: Ядро игрового процесса (без редактора)

pub struct GameLoop {
    pub is_running: bool,
}

impl GameLoop {
    pub fn new() -> Self {
        Self { is_running: false }
    }
}
