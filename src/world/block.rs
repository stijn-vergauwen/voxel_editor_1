use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Reflect, Default)]
pub struct Block {
    pub color: Color,
}

impl Block {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
