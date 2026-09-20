//! GPU контекст — устройство и очередь команд

/// Контекст GPU — хранит устройство и очередь для отрисовки
pub struct GpuContext {
    /// Устройство (создаёт ресурсы)
    pub device: wgpu::Device,
    /// Очередь (отправляет команды на GPU)
    pub queue: wgpu::Queue,
    /// Формат поверхности (обычно BGRA8UnormSrgb)
    pub surface_format: wgpu::TextureFormat,
}

impl GpuContext {
    /// Создаёт GPU контекст из уже существующих устройства и очереди
    /// (используется при интеграции с eframe)
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            device,
            queue,
            surface_format,
        }
    }
}
