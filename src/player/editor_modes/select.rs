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
                    handle_selection_input,
                    draw_current_selection,
                    handle_color_change_input,
                    delete_selection_on_keypress,
                )
                    .run_if(in_state(EditorMode::Select)),
            )
            .add_systems(OnExit(EditorMode::Select), clear_current_selection);
    }
}

const CLEAR_SELECTION_KEY: KeyCode = KeyCode::Delete;

#[derive(Resource, Debug, Default)]
pub struct CurrentSelection {
    coordinates: Vec<Coordinate>,
}

impl CurrentSelection {
    fn clear_selection(&mut self) {
        self.coordinates.clear();
    }
}

fn handle_selection_input(
    mut on_mouse_pressed: EventReader<OnMousePressed>,
    mut current_selection: ResMut<CurrentSelection>,
) {
    for target in on_mouse_pressed.iter().filter_map(|mouse_pressed| {
        (mouse_pressed.button == MouseButton::Left).then_some(mouse_pressed.target?)
    }) {
        toggle_coordinate_in_selection(target.in_coord, &mut current_selection);
    }
}

fn handle_color_change_input(
    current_selection: Res<CurrentSelection>,
    mut on_color_clicked: EventReader<OnColorClicked>,
    mut chunks: Query<&mut Chunk>,
) {
    let mut chunk = chunks.single_mut();

    for color_clicked in on_color_clicked.iter() {
        let color = color_clicked.color;

        apply_color_to_selection(color, &current_selection, &mut chunk);
    }
}

fn delete_selection_on_keypress(
    input: Res<Input<KeyCode>>,
    current_selection: ResMut<CurrentSelection>,
    mut chunks: Query<&mut Chunk>,
) {
    if input.just_pressed(CLEAR_SELECTION_KEY) {
        let mut chunk = chunks.single_mut();

        delete_selected_blocks(&current_selection, &mut chunk);
        clear_current_selection(current_selection);
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

fn apply_color_to_selection(color: Color, current_selection: &CurrentSelection, chunk: &mut Chunk) {
    for coord in current_selection.coordinates.iter() {
        if chunk.get_block(*coord).is_some() {
            chunk.set_block(*coord, Some(Block::new(color)));
        }
    }
}

fn delete_selected_blocks(current_selection: &CurrentSelection, chunk: &mut Chunk) {
    for coord in current_selection.coordinates.iter() {
        chunk.set_block(*coord, None);
    }
}

fn clear_current_selection(mut current_selection: ResMut<CurrentSelection>) {
    current_selection.clear_selection();
}

// Gizmos

fn draw_current_selection(
    current_selection: Res<CurrentSelection>,
    mut gizmos: Gizmos,
    world_settings: Res<WorldSettings>,
) {
    for coord in current_selection.coordinates.iter() {
        let in_position = world_settings.coordinate_to_position(*coord);

        gizmos.cuboid(Transform::from_translation(in_position), Color::FUCHSIA);
    }
}
