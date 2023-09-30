use bevy::prelude::*;

pub struct ColorLibraryPlugin;

impl Plugin for ColorLibraryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ColorLibrary::with_default_colors());
    }
}

#[derive(Resource)]
pub struct ColorLibrary {
    selected_color: Option<Color>,
    colors: Vec<Color>,
}

impl ColorLibrary {
    pub fn empty() -> Self {
        Self {
            colors: Vec::new(),
            selected_color: None,
        }
    }

    pub fn with_default_colors() -> Self {
        Self {
            colors: vec![Color::LIME_GREEN, Color::CYAN, Color::GRAY],
            selected_color: None,
        }
    }

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
        self.selected_color
    }

    pub fn select_color(&mut self, color: Color) {
        self.selected_color = Some(color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Deselect color

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
}
