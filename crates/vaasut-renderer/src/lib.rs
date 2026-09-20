//! Vaasut Renderer: Рендеринг 2D и 3D графики через wgpu

pub mod device;
pub mod pipeline;
pub mod camera;
pub mod geometry;
pub mod shaders;
pub mod material;

// Реэкспортируем основные типы
pub use device::GpuContext;
pub use pipeline::{RenderPipeline2D, RenderPipeline3D};
pub use camera::{Camera2DView, Camera3DView};
pub use geometry::{Vertex, MeshData};
pub use shaders::ShaderLoader;

/// Главный рендерер
pub struct Renderer {
    /// Контекст GPU (устройство, очередь)
    pub gpu: GpuContext,
    /// Конвейер для 2D
    pub pipeline_2d: Option<RenderPipeline2D>,
    /// Конвейер для 3D
    pub pipeline_3d: Option<RenderPipeline3D>,
}

impl Renderer {
    /// Создаёт новый рендерер
    pub fn new(gpu: GpuContext) -> Self {
        Self {
            gpu,
            pipeline_2d: None,
            pipeline_3d: None,
        }
    }
    
    /// Инициализирует рендерер (создаёт конвейеры)
    pub fn init(&mut self) {
        log::info!("Initializing Vaasut Renderer...");
        
        // Создаём 2D конвейер
        match RenderPipeline2D::new(&self.gpu) {
            Ok(pipeline) => {
                self.pipeline_2d = Some(pipeline);
                log::info!("2D pipeline created successfully");
            }
            Err(e) => {
                log::error!("Failed to create 2D pipeline: {:?}", e);
            }
        }
        
        log::info!("Renderer initialized!");
    }
    
    /// Рендерит сцену
    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        // Начинаем рендер-проход
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Vaasut Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.15,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        
        // Рисуем 2D объекты
        if let Some(pipeline) = &self.pipeline_2d {
            pipeline.render(&mut render_pass);
        }
    }
}
