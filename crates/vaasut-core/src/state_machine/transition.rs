//! Переходы между состояниями

/// Условие перехода — функция, которая решает, можно ли переходить
pub type TransitionCondition = Box<dyn FnMut() -> bool>;

/// Переход из одного состояния в другое
pub struct Transition {
    /// Имя состояния, ИЗ которого переходим
    pub from: String,
    /// Имя состояния, В которое переходим
    pub to: String,
    /// Условие перехода (если true — переходим)
    pub condition: TransitionCondition,
    /// Приоритет (если несколько переходов возможны, выбирается с большим)
    pub priority: i32,
}

impl Transition {
    /// Создаёт новый переход
    pub fn new(
        from: &str,
        to: &str,
        condition: impl FnMut() -> bool + 'static,
    ) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            condition: Box::new(condition),
            priority: 0,
        }
    }
    
    /// Создаёт переход с приоритетом
    pub fn with_priority(
        from: &str,
        to: &str,
        condition: impl FnMut() -> bool + 'static,
        priority: i32,
    ) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            condition: Box::new(condition),
            priority,
        }
    }
    
    /// Проверяет условие перехода
    pub fn check(&mut self) -> bool {
        (self.condition)()
    }
    
    /// Проверяет, применим ли переход из данного состояния
    pub fn is_applicable(&self, current_state: &str) -> bool {
        self.from == current_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transition_condition() {
        let mut counter = 0;
        
        let mut transition = Transition::new(
            "idle",
            "running",
            move || {
                counter += 1;
                counter >= 3
            },
        );
        
        assert!(!transition.check());
        assert!(!transition.check());
        assert!(transition.check());
    }
    
    #[test]
    fn test_transition_applicable() {
        let transition = Transition::new("idle", "running", || true);
        
        assert!(transition.is_applicable("idle"));
        assert!(!transition.is_applicable("jumping"));
    }
}
