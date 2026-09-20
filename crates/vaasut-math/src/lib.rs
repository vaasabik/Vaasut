//! Vaasut Math: Математика для игр
//! 
//! Этот модуль предоставляет математические типы и утилиты для игрового движка.
//! Базовые типы (Vec2, Vec3, Mat4, Quat) переэкспортируются из glam для производительности.
//! Специфичные типы (Transform, Color, Rect) написаны специально для Vaasut.

// Переэкспортируем базовые типы из glam
pub use glam::{
    Vec2, Vec3, Vec4,
    IVec2, IVec3, IVec4,
    UVec2, UVec3, UVec4,
    Mat2, Mat3, Mat4,
    Quat,
    vec2, vec3, vec4,
    ivec2, ivec3, ivec4,
    uvec2, uvec3, uvec4,
    mat2, mat3, mat4,
    quat,
};

// Наши модули с обёртками и утилитами
pub mod transforms;
pub mod types;
pub mod utils;

// Переэкспортируем наши типы для удобства
pub use transforms::{Transform2D, Transform3D};
pub use types::colors::Color;
pub use utils::Rect;

/// Константы для математических вычислений
pub mod constants {
    pub const PI: f32 = std::f32::consts::PI;
    pub const TAU: f32 = PI * 2.0;
    pub const DEG_TO_RAD: f32 = PI / 180.0;
    pub const RAD_TO_DEG: f32 = 180.0 / PI;
    pub const EPSILON: f32 = 1e-6;
}

/// Утилиты для работы с углами
pub mod angle {
    use super::constants::{DEG_TO_RAD, RAD_TO_DEG};
    
    /// Конвертирует градусы в радианы
    #[inline]
    pub fn deg_to_rad(degrees: f32) -> f32 {
        degrees * DEG_TO_RAD
    }
    
    /// Конвертирует радианы в градусы
    #[inline]
    pub fn rad_to_deg(radians: f32) -> f32 {
        radians * RAD_TO_DEG
    }
    
    /// Нормализует угол в диапазон [0, 360)
    #[inline]
    pub fn normalize_angle_degrees(angle: f32) -> f32 {
        let mut a = angle % 360.0;
        if a < 0.0 {
            a += 360.0;
        }
        a
    }
    
    /// Нормализует угол в диапазон [0, 2π)
    #[inline]
    pub fn normalize_angle_radians(angle: f32) -> f32 {
        use super::constants::TAU;
        let mut a = angle % TAU;
        if a < 0.0 {
            a += TAU;
        }
        a
    }
}

/// Утилиты для интерполяции
pub mod lerp {
    /// Линейная интерполяция между двумя значениями
    #[inline]
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }
    
    /// Обратная линейная интерполяция (находит t для заданного значения)
    #[inline]
    pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
        if (b - a).abs() < super::constants::EPSILON {
            0.0
        } else {
            (value - a) / (b - a)
        }
    }
    
    /// Плавная интерполяция (smoothstep)
    #[inline]
    pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }
    
    /// Ограничивает значение в диапазоне [min, max]
    #[inline]
    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
        value.max(min).min(max)
    }
}

/// Утилиты для сравнения чисел с плавающей точкой
pub mod float {
    use super::constants::EPSILON;
    
    /// Проверяет, равны ли два f32 с учётом погрешности
    #[inline]
    pub fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }
    
    /// Проверяет, равны ли два f32 с заданной точностью
    #[inline]
    pub fn approx_eq_with_tolerance(a: f32, b: f32, tolerance: f32) -> bool {
        (a - b).abs() < tolerance
    }
    
    /// Проверяет, равно ли число нулю с учётом погрешности
    #[inline]
    pub fn approx_zero(a: f32) -> bool {
        a.abs() < EPSILON
    }
}
