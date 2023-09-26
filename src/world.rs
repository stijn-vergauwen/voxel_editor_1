use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world).add_systems(Update, redraw_changed_voxels);
    }
}

// TODO: make world size configurable.
// TODO: make block struct
// TODO: replace voxel ids array with blocks array, store it as block structs instead of ids. (don't optimize prematurely)

#[derive(Component)]
pub struct World {
    pub voxel_ids: [[[u8; 16]; 16]; 16],
    pub voxels_changed: bool,
}

impl World {
    pub const EMPTY: Self = Self {
        voxel_ids: [[[0; 16]; 16]; 16],
        voxels_changed: false,
    };
}

fn spawn_world(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), generate_flat_world(2)));
}

fn generate_flat_world(ground_height: usize) -> World {
    let mut world = World::EMPTY;

    for x in 0..16 {
        for y in 0..ground_height {
            for z in 0..16 {
                world.voxel_ids[x][y][z] = 1;
            }
        }
    }
    world.voxels_changed = true;

    world
}

fn redraw_changed_voxels(
    mut commands: Commands,
    mut worlds: Query<(Entity, &mut World)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (world_entity, mut world) in worlds.iter_mut().filter(|(_, world)| world.voxels_changed) {
        println!("Update world data");

        let mesh_handle = meshes.add(shape::Cube::new(1.0).into());
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::LIME_GREEN,
            ..default()
        });

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if world.voxel_ids[x][y][z] == 1 {
                        let block = commands
                            .spawn(PbrBundle {
                                mesh: mesh_handle.clone(),
                                material: material_handle.clone(),
                                transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                                ..default()
                            })
                            .id();

                        commands.entity(world_entity).add_child(block);
                    }
                }
            }
        }

        world.voxels_changed = false;
    }
}
