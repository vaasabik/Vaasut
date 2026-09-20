//! Прямоугольник (AABB для 2D)

use crate::Vec2;

/// Прямоугольник, заданный позицией и размером
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// Создаёт новый прямоугольник
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    
    /// Создаёт прямоугольник из двух точек (min и max)
    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        Self {
            x: min.x,
            y: min.y,
            width: max.x - min.x,
            height: max.y - min.y,
        }
    }
    
    /// Создаёт прямоугольник из центра и размера
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        Self {
            x: center.x - size.x / 2.0,
            y: center.y - size.y / 2.0,
            width: size.x,
            height: size.y,
        }
    }
    
    /// Возвращает левую границу
    pub fn left(&self) -> f32 {
        self.x
    }
    
    /// Возвращает правую границу
    pub fn right(&self) -> f32 {
        self.x + self.width
    }
    
    /// Возвращает верхнюю границу
    pub fn top(&self) -> f32 {
        self.y
    }
    
    /// Возвращает нижнюю границу
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
    
    /// Возвращает центр прямоугольника
    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x + self.width / 2.0,
            self.y + self.height / 2.0,
        )
    }
    
    /// Возвращает размер прямоугольника
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
    
    /// Возвращает минимальную точку (левый верхний угол)
    pub fn min(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    
    /// Возвращает максимальную точку (правый нижний угол)
    pub fn max(&self) -> Vec2 {
        Vec2::new(self.x + self.width, self.y + self.height)
    }
    
    /// Проверяет, содержит ли прямоугольник точку
    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.x && point.x <= self.x + self.width &&
        point.y >= self.y && point.y <= self.y + self.height
    }
    
    /// Проверяет, пересекается ли с другим прямоугольником
    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
    
    /// Возвращает пересечение двух прямоугольников (или None, если не пересекаются)
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);
        
        if x1 < x2 && y1 < y2 {
            Some(Rect::new(x1, y1, x2 - x1, y2 - y1))
        } else {
            None
        }
    }
    
    /// Возвращает объединение двух прямоугольников
    pub fn union(&self, other: &Rect) -> Rect {
        let x1 = self.x.min(other.x);
        let y1 = self.y.min(other.y);
        let x2 = (self.x + self.width).max(other.x + other.width);
        let y2 = (self.y + self.height).max(other.y + other.height);
        
        Rect::new(x1, y1, x2 - x1, y2 - y1)
    }
    
    /// Расширяет прямоугольник на заданное значение со всех сторон
    pub fn expand(&self, amount: f32) -> Rect {
        Rect {
            x: self.x - amount,
            y: self.y - amount,
            width: self.width + amount * 2.0,
            height: self.height + amount * 2.0,
        }
    }
    
    /// Смещает прямоугольник
    pub fn offset(&self, dx: f32, dy: f32) -> Rect {
        Rect {
            x: self.x + dx,
            y: self.y + dy,
            width: self.width,
            height: self.height,
        }
    }
    
    /// Проверяет, пуст ли прямоугольник (нулевая или отрицательная площадь)
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
    
    /// Возвращает площадь прямоугольника
    pub fn area(&self) -> f32 {
        self.width * self.height
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rect_new() {
        let r = Rect::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(r.x, 10.0);
        assert_eq!(r.y, 20.0);
        assert_eq!(r.width, 100.0);
        assert_eq!(r.height, 50.0);
    }
    
    #[test]
    fn test_rect_center() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert_eq!(r.center(), Vec2::new(50.0, 50.0));
    }
    
    #[test]
    fn test_rect_contains_point() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        assert!(r.contains_point(Vec2::new(50.0, 50.0)));
        assert!(!r.contains_point(Vec2::new(150.0, 50.0)));
    }
    
    #[test]
    fn test_rect_intersects() {
        let r1 = Rect::new(0.0, 0.0, 100.0, 100.0);
        let r2 = Rect::new(50.0, 50.0, 100.0, 100.0);
        let r3 = Rect::new(200.0, 200.0, 50.0, 50.0);
        
        assert!(r1.intersects(&r2));
        assert!(!r1.intersects(&r3));
    }
}
