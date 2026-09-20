//! Загрузчик шейдеров

/// Загрузчик шейдеров из файлов или строк
pub struct ShaderLoader;

impl ShaderLoader {
    /// Загружает шейдер из строки
    pub fn from_source(device: &wgpu::Device, source: &str, label: &str) -> wgpu::ShaderModule {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        })
    }
    
    /// Загружает встроенный шейдер треугольника
    pub fn triangle_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
        let source = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;
        Self::from_source(device, source, "triangle_shader")
    }
}
