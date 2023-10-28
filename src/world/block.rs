use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Block {
    pub color: Color,
}

impl Block {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
