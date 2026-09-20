/// Система логирования
pub struct Logger;

impl Logger {
    pub fn info(msg: &str) {
        println!("[INFO] {}", msg);
    }
    
    pub fn error(msg: &str) {
        println!("[ERROR] {}", msg);
    }
}
