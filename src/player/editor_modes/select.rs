use bevy::prelude::*;

use crate::{
    game_systems::color_library::OnColorClicked,
    newtypes::coordinate::Coordinate,
    player::mouse_interaction::mouse_events::{OnMousePressed, OnMouseReleased},
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
                    handle_start_selecting,
                    handle_stop_selecting,
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
    start_coord: Option<Coordinate>,
    coordinates: Vec<Coordinate>,
}

impl CurrentSelection {
    fn clear_selection(&mut self) {
        self.coordinates.clear();
    }
}

// It would be nice if dragging was part of mouse_interaction, and there was an event containing all the relevant data for this action.

fn handle_start_selecting(
    mut on_mouse_pressed: EventReader<OnMousePressed>,
    mut current_selection: ResMut<CurrentSelection>,
) {
    for mouse_pressed in on_mouse_pressed
        .iter()
        .filter(|mouse_pressed| mouse_pressed.button == MouseButton::Left)
    {
        current_selection.start_coord = mouse_pressed.target.map(|target| target.in_coord);
    }
}

fn handle_stop_selecting(
    mut on_mouse_released: EventReader<OnMouseReleased>,
    mut current_selection: ResMut<CurrentSelection>,
) {
    for mouse_released in on_mouse_released
        .iter()
        .filter(|event| event.button == MouseButton::Left)
    {
        let start = current_selection.start_coord;
        let end = mouse_released.target.map(|target| target.in_coord);

        update_selection(&mut current_selection, start, end);
    }
}

fn update_selection(
    current_selection: &mut CurrentSelection,
    start: Option<Coordinate>,
    end: Option<Coordinate>,
) {
    current_selection.start_coord = None;

    if let (Some(start), Some(end)) = (start, end) {
        if start == end {
            toggle_coordinate_in_selection(start, current_selection);
        } else {
            select_group(current_selection, start, end);
        }
    }
}

fn select_group(current_selection: &mut CurrentSelection, start: Coordinate, end: Coordinate) {
    current_selection.coordinates = get_coordinates_between(start, end);
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

// TODO: split this into a function to select coord if not yet selected

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

fn get_coordinates_between(start: Coordinate, end: Coordinate) -> Vec<Coordinate> {
    let mut result = Vec::new();

    // MinMax objects with variant for coords would be nice.

    for x in start.x.min(end.x)..=start.x.max(end.x) {
        for y in start.y.min(end.y)..=start.y.max(end.y) {
            for z in start.z.min(end.z)..=start.z.max(end.z) {
                result.push(Coordinate::new(x, y, z));
            }
        }
    }

    result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_all_coordinates_between_two_coordinates() {
        let start = Coordinate::new(2, 3, 2);
        let end = Coordinate::new(4, 5, 4);

        let coords = get_coordinates_between(start, end);

        assert_eq!(coords.len(), 27);
        assert_eq!(coords.last().cloned(), Some(Coordinate::new(4, 5, 4)));
        assert!(!coords.contains(&Coordinate::new(5, 4, 3)));
    }
}
