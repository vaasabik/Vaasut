//! Компоненты физики (заготовка для будущей реализации)

use vaasut_math::{Vec2, Vec3};

/// 2D коллайдер
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Collider2D {
    /// Прямоугольник (ширина, высота)
    Rectangle(f32, f32),
    /// Круг (радиус)
    Circle(f32),
    /// Капсула (радиус, длина)
    Capsule(f32, f32),
}

/// 3D коллайдер
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Collider3D {
    /// Куб (размер по каждой оси)
    Box(Vec3),
    /// Сфера (радиус)
    Sphere(f32),
    /// Капсула (радиус, длина)
    Capsule(f32, f32),
    /// Выпуклый меш (для сложных форм)
    Mesh,
}

/// Тело с физикой 2D
#[derive(Debug, Clone, Copy)]
pub struct RigidBody2D {
    /// Масса
    pub mass: f32,
    /// Скорость
    pub velocity: Vec2,
    /// Угловая скорость
    pub angular_velocity: f32,
    /// Гравитация влияет на тело
    pub use_gravity: bool,
    /// Тело статично (не двигается)
    pub is_static: bool,
    /// Отскок (0.0 - нет отскока, 1.0 - полный отскок)
    pub restitution: f32,
    /// Трение
    pub friction: f32,
}

impl Default for RigidBody2D {
    fn default() -> Self {
        Self {
            mass: 1.0,
            velocity: Vec2::ZERO,
            angular_velocity: 0.0,
            use_gravity: true,
            is_static: false,
            restitution: 0.0,
            friction: 0.5,
        }
    }
}

/// Тело с физикой 3D
#[derive(Debug, Clone, Copy)]
pub struct RigidBody3D {
    /// Масса
    pub mass: f32,
    /// Скорость
    pub velocity: Vec3,
    /// Угловая скорость
    pub angular_velocity: Vec3,
    /// Гравитация влияет на тело
    pub use_gravity: bool,
    /// Тело статично (не двигается)
    pub is_static: bool,
    /// Отскок
    pub restitution: f32,
    /// Трение
    pub friction: f32,
}

impl Default for RigidBody3D {
    fn default() -> Self {
        Self {
            mass: 1.0,
            velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            use_gravity: true,
            is_static: false,
            restitution: 0.0,
            friction: 0.5,
        }
    }
}
