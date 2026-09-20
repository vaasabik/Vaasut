//! Компоненты для 3D мешей

use vaasut_math::Color;
use super::sprite::TextureId;

/// Идентификатор меша в системе ассетов
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshId(pub u64);

/// Идентификатор материала
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u64);

/// 3D меш — основной компонент для 3D графики
#[derive(Debug, Clone)]
pub struct MeshComponent {
    /// ID меша (геометрия)
    pub mesh: MeshId,
    /// ID материала (как рисовать)
    pub material: MaterialId,
}

impl MeshComponent {
    pub fn new(mesh: MeshId, material: MaterialId) -> Self {
        Self { mesh, material }
    }
}

/// Тип примитива (встроенные фигуры)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    /// Куб
    Cube,
    /// Плоскость
    Plane,
    /// Сфера
    Sphere,
    /// Цилиндр
    Cylinder,
    /// Конус
    Cone,
    /// Капсула
    Capsule,
}

/// Примитив — встроенная фигура (не требует загрузки)
#[derive(Debug, Clone)]
pub struct Primitive {
    pub primitive_type: PrimitiveType,
    pub color: Color,
}

impl Primitive {
    pub fn cube(color: Color) -> Self {
        Self {
            primitive_type: PrimitiveType::Cube,
            color,
        }
    }
    
    pub fn plane(color: Color) -> Self {
        Self {
            primitive_type: PrimitiveType::Plane,
            color,
        }
    }
    
    pub fn sphere(color: Color) -> Self {
        Self {
            primitive_type: PrimitiveType::Sphere,
            color,
        }
    }
}

/// Материал — определяет, как объект выглядит
#[derive(Debug, Clone)]
pub struct MaterialComponent {
    /// Основной цвет (альбедо)
    pub albedo: Color,
    /// Металличность (0.0 - диэлектрик, 1.0 - металл)
    pub metallic: f32,
    /// Шероховатость (0.0 - гладкий, 1.0 - шершавый)
    pub roughness: f32,
    /// Свечение (эмиссия)
    pub emission: Color,
    /// Прозрачность
    pub opacity: f32,
    /// ID текстуры альбедо
    pub albedo_texture: Option<TextureId>,
    /// ID текстуры нормалей
    pub normal_texture: Option<TextureId>,
}

impl MaterialComponent {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo,
            metallic: 0.0,
            roughness: 0.5,
            emission: Color::BLACK,
            opacity: 1.0,
            albedo_texture: None,
            normal_texture: None,
        }
    }
    
    pub fn with_metallic(mut self, metallic: f32) -> Self {
        self.metallic = metallic;
        self
    }
    
    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.roughness = roughness;
        self
    }
    
    pub fn with_emission(mut self, emission: Color) -> Self {
        self.emission = emission;
        self
    }
    
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }
}

impl Default for MaterialComponent {
    fn default() -> Self {
        Self::new(Color::WHITE)
    }
}
