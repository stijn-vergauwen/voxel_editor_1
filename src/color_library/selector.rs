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
        select_next_color(&mut color_library);

        println!(
            "Color '{:?}' is now selected.",
            color_library.selected_color()
        );
    }
}

fn select_next_color(library: &mut ColorLibrary) {
    library.select_color(library.colors[get_next_index_to_select(library)]);
}

fn get_next_index_to_select(library: &ColorLibrary) -> usize {
    let current_index = library
        .colors
        .iter()
        .position(|color| Some(*color) == library.selected_color());

    current_index.map_or(0, |current| (current + 1) % library.count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color_library::*;

    #[test]
    fn can_select_next_color() {
        let mut library = create_rgb_library();
        library.select_color(Color::RED);

        let first_color = library.selected_color();

        select_next_color(&mut library);

        let next_color = library.selected_color();

        assert_eq!(first_color, Some(Color::RED));
        assert_eq!(next_color, Some(Color::GREEN));
    }

    #[test]
    fn can_loop_back_to_first_color() {
        let mut library = create_rgb_library();
        library.select_color(Color::GREEN);

        select_next_color(&mut library);

        let next_color = library.selected_color();

        select_next_color(&mut library);

        let looped_to_first = library.selected_color();

        assert_eq!(next_color, Some(Color::BLUE));
        assert_eq!(looped_to_first, Some(Color::RED));
    }

    fn create_rgb_library() -> ColorLibrary {
        let mut library = ColorLibrary::empty();

        library.add_color(Color::RED);
        library.add_color(Color::GREEN);
        library.add_color(Color::BLUE);

        library
    }
}
