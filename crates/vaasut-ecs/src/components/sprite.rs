//! Компоненты для 2D спрайтов

use vaasut_math::{Vec2, Rect, Color};

/// Идентификатор текстуры в системе ассетов
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureId(pub u64);

/// 2D спрайт — основной компонент для 2D графики
#[derive(Debug, Clone)]
pub struct Sprite {
    /// ID текстуры
    pub texture: TextureId,
    /// Область текстуры (для спрайт-листов/атласов)
    pub rect: Rect,
    /// Цвет тонировки (умножается с текстурой)
    pub color: Color,
    /// Флип по горизонтали
    pub flip_x: bool,
    /// Флип по вертикали
    pub flip_y: bool,
}

impl Sprite {
    pub fn new(texture: TextureId) -> Self {
        Self {
            texture,
            rect: Rect::new(0.0, 0.0, 1.0, 1.0), // Вся текстура
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
        }
    }
    
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    
    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }
    
    pub fn with_flip(mut self, flip_x: bool, flip_y: bool) -> Self {
        self.flip_x = flip_x;
        self.flip_y = flip_y;
        self
    }
}

/// Спрайт-лист (атлас с несколькими спрайтами)
#[derive(Debug, Clone)]
pub struct SpriteSheet {
    /// ID текстуры атласа
    pub texture: TextureId,
    /// Размер одной ячейки
    pub cell_size: Vec2,
    /// Количество колонок
    pub columns: u32,
    /// Количество строк
    pub rows: u32,
}

impl SpriteSheet {
    pub fn new(texture: TextureId, cell_size: Vec2, columns: u32, rows: u32) -> Self {
        Self { texture, cell_size, columns, rows }
    }
    
    /// Получает Rect для конкретного спрайта в атласе
    pub fn get_rect(&self, index: u32) -> Rect {
        let col = index % self.columns;
        let row = index / self.columns;
        
        Rect::new(
            col as f32 * self.cell_size.x,
            row as f32 * self.cell_size.y,
            self.cell_size.x,
            self.cell_size.y,
        )
    }
}

/// Анимация спрайтов (переключение кадров)
#[derive(Debug, Clone)]
pub struct SpriteAnimation {
    /// Название анимации
    pub name: String,
    /// Индексы кадров в спрайт-листе
    pub frames: Vec<u32>,
    /// Время на один кадр (в секундах)
    pub frame_duration: f32,
    /// Зациклена ли анимация
    pub looping: bool,
    /// Текущий кадр
    pub current_frame: usize,
    /// Накопленное время
    pub elapsed: f32,
}

impl SpriteAnimation {
    pub fn new(name: &str, frames: Vec<u32>, frame_duration: f32, looping: bool) -> Self {
        Self {
            name: name.to_string(),
            frames,
            frame_duration,
            looping,
            current_frame: 0,
            elapsed: 0.0,
        }
    }
    
    /// Обновляет анимацию и возвращает текущий кадр
    pub fn update(&mut self, delta_time: f32) -> u32 {
        self.elapsed += delta_time;
        
        if self.elapsed >= self.frame_duration {
            self.elapsed -= self.frame_duration;
            self.current_frame += 1;
            
            if self.current_frame >= self.frames.len() {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames.len() - 1;
                }
            }
        }
        
        self.frames[self.current_frame]
    }
    
    /// Сбрасывает анимацию
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.elapsed = 0.0;
    }
}
