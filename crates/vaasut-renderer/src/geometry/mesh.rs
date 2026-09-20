//! Данные мешей

use super::vertex::Vertex;

/// Данные меша — вершины и индексы
#[derive(Debug, Clone)]
pub struct MeshData {
    /// Вершины
    pub vertices: Vec<Vertex>,
    /// Индексы (какие вершины соединять в треугольники)
    pub indices: Vec<u32>,
}

impl MeshData {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self { vertices, indices }
    }
    
    /// Создаёт простой треугольник
    pub fn triangle() -> Self {
        let vertices = vec![
            Vertex::new([0.0, 0.5], [1.0, 0.0, 0.0, 1.0]),   // Красный (верх)
            Vertex::new([-0.5, -0.5], [0.0, 1.0, 0.0, 1.0]),  // Зелёный (лево)
            Vertex::new([0.5, -0.5], [0.0, 0.0, 1.0, 1.0]),   // Синий (право)
        ];
        
        let indices = vec![0, 1, 2];
        
        Self { vertices, indices }
    }
    
    /// Создаёт квадрат (два треугольника)
    pub fn quad() -> Self {
        let vertices = vec![
            Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0, 1.0]),   // Верх-лево
            Vertex::new([-0.5, -0.5], [1.0, 1.0, 1.0, 1.0]),  // Низ-лево
            Vertex::new([0.5, -0.5], [1.0, 1.0, 1.0, 1.0]),   // Низ-право
            Vertex::new([0.5, 0.5], [1.0, 1.0, 1.0, 1.0]),    // Верх-право
        ];
        
        let indices = vec![0, 1, 2, 0, 2, 3];
        
        Self { vertices, indices }
    }
    
    /// Создаёт цветной квадрат
    pub fn colored_quad(color: [f32; 4]) -> Self {
        let vertices = vec![
            Vertex::new([-0.5, 0.5], color),
            Vertex::new([-0.5, -0.5], color),
            Vertex::new([0.5, -0.5], color),
            Vertex::new([0.5, 0.5], color),
        ];
        
        let indices = vec![0, 1, 2, 0, 2, 3];
        
        Self { vertices, indices }
    }
}
