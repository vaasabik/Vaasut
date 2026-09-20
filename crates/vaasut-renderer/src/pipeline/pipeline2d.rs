//! 2D конвейер рендеринга

use crate::device::GpuContext;
use crate::geometry::Vertex;
use crate::shaders::ShaderLoader;
use wgpu::util::DeviceExt; // ← Исправление 1: импорт трейта

/// Конвейер для отрисовки 2D графики
pub struct RenderPipeline2D {
    /// Графический конвейер
    pub pipeline: wgpu::RenderPipeline,
    /// Буфер вершин
    pub vertex_buffer: wgpu::Buffer,
    /// Буфер индексов
    pub index_buffer: wgpu::Buffer,
    /// Количество вершин
    pub vertex_count: u32,
    /// Количество индексов
    pub index_count: u32,
}

impl RenderPipeline2D {
    /// Создаёт новый 2D конвейер
    pub fn new(gpu: &GpuContext) -> Result<Self, wgpu::Error> {
        // Загружаем шейдер
        let shader = ShaderLoader::triangle_shader(&gpu.device);
        
        // Создаём графический конвейер
        let pipeline = gpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("2D Render Pipeline"),
            layout: None, // Автоматический
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(), // ← Исправление 2: новое поле
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: gpu.surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(), // ← Исправление 2: новое поле
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None, // Не отбрасываем полигоны для 2D
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None, // Нет глубины для 2D
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        
        // Создаём тестовый треугольник
        let mesh = crate::geometry::MeshData::triangle();
        
        // Буфер вершин
        let vertex_buffer = gpu.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&mesh.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        
        // Буфер индексов
        let index_buffer = gpu.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        
        Ok(Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            vertex_count: mesh.vertices.len() as u32,
            index_count: mesh.indices.len() as u32,
        })
    }
    
    /// Рисует объекты в рендер-проходе
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
