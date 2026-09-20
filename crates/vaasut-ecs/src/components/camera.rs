//! Компоненты камер

use vaasut_math::Color;

/// 2D камера (ортографическая)
#[derive(Debug, Clone)]
pub struct Camera2D {
    /// Масштаб камеры (1.0 = нормальный, 2.0 = приближено в 2 раза)
    pub zoom: f32,
    /// Цвет фона
    pub background_color: Color,
    /// Активна ли камера
    pub active: bool,
    /// Приоритет (если несколько камер, активна с большим приоритетом)
    pub priority: i32,
}

impl Camera2D {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            background_color: Color::rgb(0.1, 0.1, 0.15),
            active: true,
            priority: 0,
        }
    }
    
    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }
    
    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self::new()
    }
}

/// 3D камера (перспективная)
#[derive(Debug, Clone)]
pub struct Camera3D {
    /// Поле зрения в градусах (обычно 60-90)
    pub fov_degrees: f32,
    /// Ближняя плоскость отсечения
    pub near_plane: f32,
    /// Дальняя плоскость отсечения
    pub far_plane: f32,
    /// Цвет фона
    pub background_color: Color,
    /// Активна ли камера
    pub active: bool,
    /// Приоритет
    pub priority: i32,
}

impl Camera3D {
    pub fn new() -> Self {
        Self {
            fov_degrees: 60.0,
            near_plane: 0.1,
            far_plane: 1000.0,
            background_color: Color::rgb(0.1, 0.1, 0.15),
            active: true,
            priority: 0,
        }
    }
    
    pub fn with_fov(mut self, fov: f32) -> Self {
        self.fov_degrees = fov;
        self
    }
    
    pub fn with_clipping_planes(mut self, near: f32, far: f32) -> Self {
        self.near_plane = near;
        self.far_plane = far;
        self
    }
    
    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }
    
    /// Возвращает соотношение сторон (пока фиксированное)
    pub fn aspect_ratio(&self) -> f32 {
        16.0 / 9.0
    }
}

impl Default for Camera3D {
    fn default() -> Self {
        Self::new()
    }
}

/// Режим рендеринга камеры
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    /// Только 2D
    D2,
    /// Только 3D
    D3,
    /// Оба (3D сначала, 2D поверх)
    Mixed,
    /// Автоматически (определяется по содержимому сцены)
    Auto,
}
