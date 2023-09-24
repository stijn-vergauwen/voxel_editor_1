use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_world);
    }
}

// TODO: make world size configurable.
// TODO: make block struct
// TODO: replace voxel ids array with blocks array, store it as block structs instead of ids. (don't optimize prematurely)

#[derive(Component)]
pub struct World {
    pub voxel_ids: [[[u8; 16]; 16]; 16],
}

impl World {
    pub const EMPTY: Self = Self {
        voxel_ids: [[[0; 16]; 16]; 16],
    };
}

fn spawn_world(mut commands: Commands) {
    commands.spawn(World::EMPTY);
}
