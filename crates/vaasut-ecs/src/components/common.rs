//! Общие компоненты для всех объектов

use vaasut_core::Name;

/// Компонент имени объекта
/// Используется для отображения в редакторе
pub type NameComponent = Name;

/// Компонент видимости
/// Если отсутствует или `visible = false`, объект не рендерится
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Visible {
    pub visible: bool,
}

impl Visible {
    pub fn new(visible: bool) -> Self {
        Self { visible }
    }
    
    pub fn shown() -> Self {
        Self { visible: true }
    }
    
    pub fn hidden() -> Self {
        Self { visible: false }
    }
}

impl Default for Visible {
    fn default() -> Self {
        Self { visible: true }
    }
}

/// Тег для группировки объектов (например, "enemy", "player", "background")
#[derive(Debug, Clone)]
pub struct Tag {
    pub tag: String,
}

impl Tag {
    pub fn new(tag: &str) -> Self {
        Self { tag: tag.to_string() }
    }
}

/// Порядок отрисовки (чем меньше, тем раньше рисуется)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RenderOrder {
    pub order: i32,
}

impl RenderOrder {
    pub fn new(order: i32) -> Self {
        Self { order }
    }
}

impl Default for RenderOrder {
    fn default() -> Self {
        Self { order: 0 }
    }
}

/// Статичный объект (не обновляется каждый кадр)
#[derive(Debug, Clone, Copy)]
pub struct Static;

/// Активный объект (обрабатывается системами)
#[derive(Debug, Clone, Copy)]
pub struct Active;
