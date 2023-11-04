use bevy::prelude::*;

use crate::{
    game_systems::color_library::OnColorClicked,
    newtypes::coordinate::Coordinate,
    player::mouse_interaction::OnMousePressed,
    world::{block::Block, chunk::Chunk, WorldSettings},
};

use super::EditorMode;

pub struct SelectModePlugin;

impl Plugin for SelectModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentSelection>()
            .add_systems(
                Update,
                (
                    handle_input,
                    draw_current_selection,
                    apply_color_to_selection,
                )
                    .run_if(in_state(EditorMode::Select)),
            )
            .add_systems(OnExit(EditorMode::Select), clear_current_selection);
    }
}

#[derive(Resource, Debug, Default)]
pub struct CurrentSelection {
    coordinates: Vec<Coordinate>,
}

impl CurrentSelection {
    fn clear_selection(&mut self) {
        self.coordinates.clear();
    }
}

fn handle_input(
    mut on_mouse_pressed: EventReader<OnMousePressed>,
    mut current_selection: ResMut<CurrentSelection>,
) {
    for target in on_mouse_pressed.iter().filter_map(|mouse_pressed| {
        (mouse_pressed.button == MouseButton::Left).then_some(mouse_pressed.target?)
    }) {
        toggle_coordinate_in_selection(target.in_coord, &mut current_selection);
    }
}

fn toggle_coordinate_in_selection(coord: Coordinate, selection: &mut CurrentSelection) {
    if selection.coordinates.contains(&coord) {
        let coordinates_iterator = selection.coordinates.clone().into_iter();
        selection.coordinates = coordinates_iterator.filter(|item| *item != coord).collect();
    } else {
        selection.coordinates.push(coord);
    }
}

fn apply_color_to_selection(
    selection: Res<CurrentSelection>,
    mut on_color_clicked: EventReader<OnColorClicked>,
    mut chunks: Query<&mut Chunk>,
) {
    let mut chunk = chunks.single_mut();

    for color_clicked in on_color_clicked.iter() {
        let color = color_clicked.color;

        for coord in selection.coordinates.iter() {
            if chunk.get_block(*coord).is_some() {
                chunk.set_block(*coord, Some(Block::new(color)));
            }
        }
    }
}

fn clear_current_selection(mut current_selection: ResMut<CurrentSelection>) {
    current_selection.clear_selection();
}

fn draw_current_selection(
    selection: Res<CurrentSelection>,
    mut gizmos: Gizmos,
    world_settings: Res<WorldSettings>,
) {
    for coord in selection.coordinates.iter() {
        let in_position = world_settings.coordinate_to_position(*coord);

        gizmos.cuboid(Transform::from_translation(in_position), Color::FUCHSIA);
    }
}
