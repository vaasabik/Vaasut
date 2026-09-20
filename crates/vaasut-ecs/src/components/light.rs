//! Компоненты источников света

use vaasut_math::{Color, Vec3};

/// Тип источника света
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    /// Направленный свет (как солнце) — светит в одном направлении
    Directional,
    /// Точечный свет (как лампочка) — светит во все стороны из точки
    Point,
    /// Прожектор (как фонарик) — светит конусом
    Spot,
    /// Окружающий свет — равномерно освещает всё
    Ambient,
}

/// Компонент источника света
#[derive(Debug, Clone)]
pub struct Light {
    /// Тип света
    pub light_type: LightType,
    /// Цвет света
    pub color: Color,
    /// Интенсивность (яркость)
    pub intensity: f32,
    /// Радиус действия (для точечных и прожекторных)
    pub range: f32,
    /// Угол конуса (для прожекторов, в радианах)
    pub spot_angle: f32,
    /// Включён ли свет
    pub enabled: bool,
}

impl Light {
    /// Создаёт направленный свет (солнце)
    pub fn directional(color: Color, intensity: f32) -> Self {
        Self {
            light_type: LightType::Directional,
            color,
            intensity,
            range: f32::MAX,
            spot_angle: 0.0,
            enabled: true,
        }
    }
    
    /// Создаёт точечный свет (лампочка)
    pub fn point(color: Color, intensity: f32, range: f32) -> Self {
        Self {
            light_type: LightType::Point,
            color,
            intensity,
            range,
            spot_angle: 0.0,
            enabled: true,
        }
    }
    
    /// Создаёт прожектор (фонарик)
    pub fn spot(color: Color, intensity: f32, range: f32, angle: f32) -> Self {
        Self {
            light_type: LightType::Spot,
            color,
            intensity,
            range,
            spot_angle: angle,
            enabled: true,
        }
    }
    
    /// Создаёт окружающий свет
    pub fn ambient(color: Color, intensity: f32) -> Self {
        Self {
            light_type: LightType::Ambient,
            color,
            intensity,
            range: f32::MAX,
            spot_angle: 0.0,
            enabled: true,
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Self::directional(Color::WHITE, 1.0)
    }
}
