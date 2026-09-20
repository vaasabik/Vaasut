/// Сцена
pub struct Scene {
    pub name: String,
}

impl Scene {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
