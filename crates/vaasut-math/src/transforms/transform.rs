//! Трансформы для позиционирования объектов в пространстве

use crate::{Vec2, Vec3, Quat, Mat3, Mat4};

/// 2D трансформ (позиция, поворот, масштаб)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2D {
    /// Позиция в 2D пространстве
    pub position: Vec2,
    /// Угол поворота в радианах
    pub rotation: f32,
    /// Масштаб по осям X и Y
    pub scale: Vec2,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }
}

impl Transform2D {
    /// Создаёт новый трансформ
    pub fn new(position: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self { position, rotation, scale }
    }
    
    /// Создаёт трансформ только с позицией
    pub fn from_position(position: Vec2) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }
    
    /// Возвращает матрицу трансформации 3x3
    pub fn to_matrix(&self) -> Mat3 {
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        
        Mat3::from_cols_array(&[
            cos * self.scale.x, sin * self.scale.x, 0.0,
            -sin * self.scale.y, cos * self.scale.y, 0.0,
            self.position.x, self.position.y, 1.0,
        ])
    }
    
    /// Трансформирует точку (применяет масштаб, поворот и перемещение)
    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        let cos = self.rotation.cos();
        let sin = self.rotation.sin();
        
        let scaled = point * self.scale;
        let rotated = Vec2::new(
            scaled.x * cos - scaled.y * sin,
            scaled.x * sin + scaled.y * cos,
        );
        
        rotated + self.position
    }
    
    /// Перемещает трансформ
    pub fn translate(&mut self, delta: Vec2) {
        self.position += delta;
    }
    
    /// Поворачивает трансформ (в радианах)
    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }
    
    /// Масштабирует трансформ
    pub fn scale(&mut self, factor: Vec2) {
        self.scale *= factor;
    }
}

/// 3D трансформ (позиция, поворот, масштаб)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3D {
    /// Позиция в 3D пространстве
    pub position: Vec3,
    /// Поворот (кватернион для избежания gimbal lock)
    pub rotation: Quat,
    /// Масштаб по осям X, Y и Z
    pub scale: Vec3,
}

impl Default for Transform3D {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Transform3D {
    /// Создаёт новый трансформ
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self { position, rotation, scale }
    }
    
    /// Создаёт трансформ только с позицией
    pub fn from_position(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }
    
    /// Создаёт трансформ с позицией и поворотом (в углах Эйлера в радианах)
    pub fn from_position_rotation(position: Vec3, euler_angles: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::from_euler(glam::EulerRot::YXZ, euler_angles.y, euler_angles.x, euler_angles.z),
            scale: Vec3::ONE,
        }
    }
    
    /// Возвращает матрицу трансформации 4x4
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }
    
    /// Трансформирует точку
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        let scaled = point * self.scale;
        let rotated = self.rotation * scaled;
        rotated + self.position
    }
    
    /// Трансформирует направление (без учёта позиции)
    pub fn transform_direction(&self, direction: Vec3) -> Vec3 {
        let scaled = direction * self.scale;
        self.rotation * scaled
    }
    
    /// Перемещает трансформ
    pub fn translate(&mut self, delta: Vec3) {
        self.position += delta;
    }
    
    /// Перемещает трансформ в локальных координатах (относительно поворота)
    pub fn translate_local(&mut self, delta: Vec3) {
        self.position += self.rotation * delta;
    }
    
    /// Поворачивает трансформ вокруг оси
    pub fn rotate(&mut self, axis: Vec3, angle: f32) {
        let delta_rotation = Quat::from_axis_angle(axis, angle);
        self.rotation = delta_rotation * self.rotation;
    }
    
    /// Поворачивает трансформ с помощью кватерниона
    pub fn rotate_quat(&mut self, rotation: Quat) {
        self.rotation = rotation * self.rotation;
    }
    
    /// Масштабирует трансформ
    pub fn scale(&mut self, factor: Vec3) {
        self.scale *= factor;
    }
    
    /// Возвращает направление "вперёд" (отрицательный Z)
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }
    
    /// Возвращает направление "вверх" (положительный Y)
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
    
    /// Возвращает направление "вправо" (положительный X)
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }
    
    /// Смотрит на точку (поворачивает трансформ лицом к цели)
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let forward = (target - self.position).normalize();
        if forward.length_squared() > crate::constants::EPSILON {
            self.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, forward);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2;
    
    #[test]
    fn test_transform2d_default() {
        let t = Transform2D::default();
        assert_eq!(t.position, Vec2::ZERO);
        assert_eq!(t.rotation, 0.0);
        assert_eq!(t.scale, Vec2::ONE);
    }
    
    #[test]
    fn test_transform2d_translate() {
        let mut t = Transform2D::default();
        t.translate(vec2(5.0, 10.0));
        assert_eq!(t.position, vec2(5.0, 10.0));
    }
    
    #[test]
    fn test_transform3d_default() {
        let t = Transform3D::default();
        assert_eq!(t.position, Vec3::ZERO);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::ONE);
    }
    
    #[test]
    fn test_transform3d_translate() {
        let mut t = Transform3D::default();
        t.translate(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.position, Vec3::new(1.0, 2.0, 3.0));
    }
}
