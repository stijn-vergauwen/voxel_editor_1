use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Block {
    pub color: Color,
}

impl Block {
    pub const GRASS: Self = Self {
        color: Color::LIME_GREEN,
    };
}
