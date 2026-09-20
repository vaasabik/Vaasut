//! Vaasut Renderer: Рендеринг 2D и 3D графики через wgpu

pub use wgpu;

/// Режим рендеринга
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMode {
    /// Рендеринг 2D спрайтов и UI
    D2,
    /// Рендеринг 3D моделей и мира
    D3,
    /// Смешанный режим (например, 3D мир + 2D интерфейс)
    Mixed,
}

/// Главный рендерер движка
pub struct Renderer {
    pub mode: RenderMode,
    // Здесь будут: wgpu::Device, wgpu::Queue, wgpu::Surface
}

impl Renderer {
    pub fn new(mode: RenderMode) -> Self {
        Self { mode }
    }
    
    /// Инициализация рендерера (здесь будет создаваться wgpu::Device)
    pub fn init(&mut self) {
        log::info!("Initializing Vaasut Renderer in {:?} mode", self.mode);
    }
}
