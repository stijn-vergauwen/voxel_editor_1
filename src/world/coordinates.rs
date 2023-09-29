use bevy::prelude::*;

// TODO: rename to coordinates

#[derive(Clone, Copy)]
pub struct ChunkIndex {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl ChunkIndex {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

impl From<Vec3> for ChunkIndex {
    fn from(value: Vec3) -> Self {
        ChunkIndex {
            x: value.x as usize,
            y: value.y as usize,
            z: value.z as usize,
        }
    }
}
