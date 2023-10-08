use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Reflect, Default)]
pub struct Block {
    pub color: Color,
}

impl Block {
    pub const GRASS: Self = Self {
        color: Color::LIME_GREEN,
    };

    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
