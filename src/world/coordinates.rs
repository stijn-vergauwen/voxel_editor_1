#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}
