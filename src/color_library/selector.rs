use bevy::prelude::*;

use super::ColorLibrary;

pub struct ColorSelectorPlugin;

impl Plugin for ColorSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_next_color_on_input);
    }
}

const NEXT_COLOR_KEY: KeyCode = KeyCode::Period;

fn select_next_color_on_input(input: Res<Input<KeyCode>>, mut color_library: ResMut<ColorLibrary>) {
    if input.just_pressed(NEXT_COLOR_KEY) {
        color_library.select_next();

        println!(
            "Color '{:?}' is now selected.",
            color_library.selected_color()
        );
    }
}
