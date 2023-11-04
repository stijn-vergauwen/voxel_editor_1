mod selector;

use bevy::prelude::*;

use crate::player::editor_modes::EditorMode;

use self::selector::ColorSelectorPlugin;

pub struct ColorLibraryPlugin;

impl Plugin for ColorLibraryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ColorSelectorPlugin)
            .add_event::<OnColorClicked>()
            .insert_resource(ColorLibrary::with_default_colors())
            .add_systems(
                Update,
                switch_to_clicked_color.run_if(in_state(EditorMode::Build)),
            );
    }
}

#[derive(Resource)]
pub struct ColorLibrary {
    selected_index: usize,
    colors: Vec<Color>,
}

impl ColorLibrary {
    #[allow(unused)]
    pub fn empty() -> Self {
        Self {
            colors: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn with_default_colors() -> Self {
        Self {
            colors: vec![Color::LIME_GREEN, Color::CYAN, Color::GRAY],
            selected_index: 0,
        }
    }

    #[allow(unused)]
    pub fn add_color(&mut self, color: Color) {
        self.colors.push(color);
    }

    pub fn all_colors(&self) -> Vec<Color> {
        self.colors.clone()
    }

    pub fn count(&self) -> usize {
        self.colors.len()
    }

    pub fn selected_color(&self) -> Option<Color> {
        self.colors.get(self.selected_index).cloned()
    }

    pub fn select_index(&mut self, index: usize) {
        if index >= self.count() {
            return;
        }

        self.selected_index = index;
    }

    pub fn select_color(&mut self, color: Color) -> bool {
        let found_index = self.find_index_of_color(color);

        if let Some(index) = found_index {
            self.select_index(index)
        }

        found_index.is_some()
    }

    pub fn select_next(&mut self) {
        self.select_index((self.selected_index + 1) % self.count());
    }

    fn find_index_of_color(&self, color: Color) -> Option<usize> {
        self.colors.iter().position(|e| *e == color)
    }
}

#[derive(Event)]
pub struct OnColorClicked {
    pub color: Color,
}

impl OnColorClicked {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

fn switch_to_clicked_color(
    mut on_clicked: EventReader<OnColorClicked>,
    mut color_library: ResMut<ColorLibrary>,
) {
    for event in on_clicked.iter() {
        color_library.select_color(event.color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_colors_count() {
        let library = ColorLibrary::empty();

        let count = library.count();

        assert_eq!(count, 0);
    }

    #[test]
    fn can_initialize_default_colors() {
        let default_colors = ColorLibrary::with_default_colors();

        let count = default_colors.count();

        assert_ne!(count, 0);
    }

    #[test]
    fn can_add_new_color() {
        let mut library = ColorLibrary::empty();
        let color = Color::LIME_GREEN;

        library.add_color(color);
        let count = library.count();

        assert_eq!(count, 1);
    }

    #[test]
    fn can_get_all_colors() {
        let mut library = ColorLibrary::empty();

        library.add_color(Color::YELLOW);
        library.add_color(Color::BLUE);

        let colors = library.all_colors();
        let second_color = colors[1];

        assert_eq!(second_color, Color::BLUE);
    }

    #[test]
    fn can_get_selected_color() {
        let library = ColorLibrary::empty();

        let selected = library.selected_color();

        assert_eq!(selected, None);
    }

    #[test]
    fn can_select_color() {
        let mut library = ColorLibrary::empty();
        let color = Color::MAROON;

        library.add_color(color);
        library.select_color(color);

        let selected_color = library.selected_color();

        assert_eq!(selected_color, Some(Color::MAROON));
    }

    #[test]
    fn can_select_next_color() {
        let mut library = create_rgb_library();
        library.select_color(Color::RED);

        let first_color = library.selected_color();

        library.select_next();

        let next_color = library.selected_color();

        assert_eq!(first_color, Some(Color::RED));
        assert_eq!(next_color, Some(Color::GREEN));
    }

    #[test]
    fn can_loop_back_to_first_color() {
        let mut library = create_rgb_library();
        library.select_color(Color::GREEN);

        library.select_next();

        let next_color = library.selected_color();

        library.select_next();

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
