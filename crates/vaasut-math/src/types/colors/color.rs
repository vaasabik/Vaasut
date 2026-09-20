//! Цвет (RGBA) с утилитами для работы

use crate::Vec4;

/// Цвет в формате RGBA (значения от 0.0 до 1.0)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    // Предопределённые цвета
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Self = Self { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const CYAN: Self = Self { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    
    /// Создаёт цвет из RGBA компонентов (0.0 - 1.0)
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    /// Создаёт цвет из RGB компонентов (0.0 - 1.0) с полной непрозрачностью
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    
    /// Создаёт цвет из RGBA значений (0-255)
    pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
    
    /// Создаёт цвет из RGB значений (0-255) с полной непрозрачностью
    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::from_rgba8(r, g, b, 255)
    }
    
    /// Создаёт цвет из шестнадцатеричной строки (например, "#FF6B35" или "FF6B35")
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        let hex = hex.trim_start_matches('#');
        
        if hex.len() != 6 && hex.len() != 8 {
            return Err("Hex string must be 6 or 8 characters (excluding #)");
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex")?;
        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16).map_err(|_| "Invalid hex")?
        } else {
            255
        };
        
        Ok(Self::from_rgba8(r, g, b, a))
    }
    
    /// Преобразует цвет в Vec4
    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }
    
    /// Преобразует цвет в массив [r, g, b, a]
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    
    /// Преобразует цвет в массив u8 [r, g, b, a] (0-255)
    pub fn to_rgba8(&self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ]
    }
    
    /// Линейная интерполяция между двумя цветами
    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        Color {
            r: crate::lerp::lerp(self.r, other.r, t),
            g: crate::lerp::lerp(self.g, other.g, t),
            b: crate::lerp::lerp(self.b, other.b, t),
            a: crate::lerp::lerp(self.a, other.a, t),
        }
    }
    
    /// Возвращает цвет с изменённой прозрачностью
    pub fn with_alpha(&self, alpha: f32) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }
    
    /// Затемняет цвет (уменьшает яркость)
    pub fn darken(&self, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color {
            r: self.r * (1.0 - factor),
            g: self.g * (1.0 - factor),
            b: self.b * (1.0 - factor),
            a: self.a,
        }
    }
    
    /// Осветляет цвет (увеличивает яркость)
    pub fn lighten(&self, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color {
            r: self.r + (1.0 - self.r) * factor,
            g: self.g + (1.0 - self.g) * factor,
            b: self.b + (1.0 - self.b) * factor,
            a: self.a,
        }
    }
    
    /// Преобразует в оттенки серого
    pub fn to_grayscale(&self) -> Color {
        let gray = 0.299 * self.r + 0.587 * self.g + 0.114 * self.b;
        Color {
            r: gray,
            g: gray,
            b: gray,
            a: self.a,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl From<Vec4> for Color {
    fn from(v: Vec4) -> Self {
        Self { r: v.x, g: v.y, b: v.z, a: v.w }
    }
}

impl From<Color> for Vec4 {
    fn from(c: Color) -> Self {
        Vec4::new(c.r, c.g, c.b, c.a)
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl From<[f32; 4]> for Color {
    fn from(arr: [f32; 4]) -> Self {
        Self { r: arr[0], g: arr[1], b: arr[2], a: arr[3] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_color_new() {
        let c = Color::new(0.5, 0.6, 0.7, 1.0);
        assert_eq!(c.r, 0.5);
        assert_eq!(c.g, 0.6);
        assert_eq!(c.b, 0.7);
        assert_eq!(c.a, 1.0);
    }
    
    #[test]
    fn test_color_from_rgb8() {
        let c = Color::from_rgb8(255, 128, 0);
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 0.5).abs() < 0.01);
        assert_eq!(c.b, 0.0);
        assert_eq!(c.a, 1.0);
    }
    
    #[test]
    fn test_color_from_hex() {
        let c = Color::from_hex("#FF6B35").unwrap();
        assert!((c.r - 1.0).abs() < 0.01);
        assert!((c.g - 0.42).abs() < 0.01);
        assert!((c.b - 0.21).abs() < 0.01);
    }
    
    #[test]
    fn test_color_lerp() {
        let a = Color::BLACK;
        let b = Color::WHITE;
        let c = a.lerp(&b, 0.5);
        assert!((c.r - 0.5).abs() < 0.01);
        assert!((c.g - 0.5).abs() < 0.01);
        assert!((c.b - 0.5).abs() < 0.01);
    }
}
