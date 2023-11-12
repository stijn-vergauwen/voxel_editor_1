use bevy::prelude::*;

use crate::{
    game_systems::color_library::OnColorClicked,
    newtypes::coordinate::Coordinate,
    player::mouse_interaction::mouse_events::{OnMouseDrag, OnMousePressed},
    world::{block::Block, chunk::Chunk, WorldSettings},
};

use super::EditorMode;

/*
    Tips for next iteration:

    - clear previous selection by default, hold a key to add to selection
    - Have a way to identify groups of blocks, like a wall, to select the full wall in one click
    - Split each action to it's own module

*/

pub struct SelectModePlugin;

impl Plugin for SelectModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentSelection>()
            .add_systems(
                Update,
                (
                    handle_mouse_pressed,
                    handle_drag_selection,
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

fn handle_mouse_pressed(
    mut on_mouse_pressed: EventReader<OnMousePressed>,
    mut current_selection: ResMut<CurrentSelection>,
) {
    for mouse_press in on_mouse_pressed
        .iter()
        .filter(|mouse_press| mouse_press.button == MouseButton::Left)
    {
        if let Some(coord) = mouse_press.target.map(|target| target.in_coord) {
            toggle_coordinate_in_selection(coord, &mut current_selection);
        }
    }
}

fn handle_drag_selection(
    mut on_mouse_drag: EventReader<OnMouseDrag>,
    mut current_selection: ResMut<CurrentSelection>,
) {
    for mouse_drag in on_mouse_drag
        .iter()
        .filter(|mouse_drag| mouse_drag.button == MouseButton::Left && mouse_drag.drag_ended())
    {
        update_selection(
            &mut current_selection,
            mouse_drag.start.map(|target| target.in_coord),
            mouse_drag.end.map(|target| target.in_coord),
        );
    }
}

fn update_selection(
    current_selection: &mut CurrentSelection,
    start: Option<Coordinate>,
    end: Option<Coordinate>,
) {
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

pub fn get_coordinates_between(start: Coordinate, end: Coordinate) -> Vec<Coordinate> {
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
