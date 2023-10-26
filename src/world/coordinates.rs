use bevy::prelude::*;

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

    pub fn max_element(&self) -> usize {
        self.x.max(self.y.max(self.z))
    }
}

impl From<Vec3> for Coordinate {
    fn from(value: Vec3) -> Self {
        Coordinate {
            x: value.x as usize,
            y: value.y as usize,
            z: value.z as usize,
        }
    }
}
