#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn square(size: u32) -> Self {
        Self::new(size, size)
    }
}
