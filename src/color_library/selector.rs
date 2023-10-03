use bevy::prelude::*;

use super::ColorLibrary;

pub struct ColorSelectorPlugin;

impl Plugin for ColorSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_selector_buttons)
            .add_systems(Update, (select_next_color_on_key, button_interaction));
    }
}

const NEXT_COLOR_KEY: KeyCode = KeyCode::Period;

#[derive(Component, Clone, Copy)]
struct SelectorButton {
    color: Color,
    pub is_highlighted: bool,
}

impl SelectorButton {
    fn new(color: Color) -> Self {
        Self {
            color,
            is_highlighted: false,
        }
    }

    fn to_ui(&self) -> (ButtonBundle, SelectorButton) {
        (
            ButtonBundle {
                background_color: self.color.into(),
                border_color: self.border_color().into(),
                style: Style {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                ..default()
            },
            self.clone(),
        )
    }

    // TODO: move to library or delete this method. doesn't make sense here
    fn set_selected(&self, library: &mut ColorLibrary) {
        library.select_color(self.color);
    }

    fn border_color(&self) -> Color {
        if self.is_highlighted {
            Color::WHITE
        } else {
            Color::DARK_GRAY
        }
    }
}

fn select_next_color_on_key(input: Res<Input<KeyCode>>, mut color_library: ResMut<ColorLibrary>) {
    if input.just_pressed(NEXT_COLOR_KEY) {
        color_library.select_next();
    }
}

fn spawn_selector_buttons(mut commands: Commands, color_library: Res<ColorLibrary>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                padding: UiRect::all(Val::Px(10.0)),
                column_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|list| {
            let buttons = build_buttons_from_colors(color_library.all_colors());
            for node in build_buttons_ui(buttons, color_library.selected_color()) {
                list.spawn(node);
            }
        });
}

fn button_interaction(
    buttons: Query<(&SelectorButton, &Interaction), Changed<Interaction>>,
    mut color_library: ResMut<ColorLibrary>,
) {
    for (button, interaction) in buttons.iter() {
        if *interaction == Interaction::Pressed {
            button.set_selected(&mut color_library);
        }
    }
}

fn build_buttons_from_colors(colors: Vec<Color>) -> Vec<SelectorButton> {
    colors
        .into_iter()
        .map(|color| SelectorButton::new(color))
        .collect()
}

fn build_buttons_ui(
    buttons: Vec<SelectorButton>,
    selected_color: Option<Color>,
) -> Vec<(ButtonBundle, SelectorButton)> {
    buttons
        .into_iter()
        .map(|mut button| {
            button.is_highlighted = Some(button.color) == selected_color;
            button.to_ui()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: selecting new color highlights that button
    // TODO:

    #[test]
    fn can_create_selector_button() {
        let selector = SelectorButton::new(Color::GREEN);

        assert_eq!(selector.color, Color::GREEN);
    }

    #[test]
    fn can_build_ui_node_from_selector_button() {
        let selector = SelectorButton::new(Color::GREEN);

        let node: (ButtonBundle, SelectorButton) = selector.to_ui();

        assert_eq!(node.0.background_color.0, Color::GREEN);
    }

    #[test]
    fn clicking_button_selects_that_color() {
        let mut library = ColorLibrary::empty();
        library.add_color(Color::RED);
        library.add_color(Color::GREEN);

        let selector = SelectorButton::new(Color::GREEN);

        assert_eq!(library.selected_color(), Some(Color::RED));

        selector.set_selected(&mut library);

        assert_eq!(library.selected_color(), Some(Color::GREEN));
    }

    #[test]
    fn can_set_button_highlighted() {
        let mut selector = SelectorButton::new(Color::GREEN);

        // This test shows that highlighted should default to false, but does it provide any value?
        assert_eq!(selector.is_highlighted, false);

        selector.is_highlighted = true;

        assert_eq!(selector.is_highlighted, true);
    }

    #[test]
    fn button_highlighted_changes_border_color() {
        let mut selector = SelectorButton::new(Color::GREEN);
        selector.is_highlighted = true;

        let node: (ButtonBundle, SelectorButton) = selector.to_ui();

        assert_eq!(node.0.border_color.0, Color::WHITE);
    }

    #[test]
    fn selected_color_gets_highlighted() {
        let buttons = build_buttons_from_colors(vec![Color::RED, Color::GREEN]);
        let nodes: Vec<(ButtonBundle, SelectorButton)> =
            build_buttons_ui(buttons, Some(Color::GREEN));

        let selected_button = &nodes[1].0;

        assert_eq!(selected_button.background_color.0, Color::GREEN);
        assert_eq!(selected_button.border_color.0, Color::WHITE);
    }
}
