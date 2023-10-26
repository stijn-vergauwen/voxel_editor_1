pub mod interaction;

use bevy::prelude::*;

use self::interaction::{OnColorClicked, SelectorInteractionPlugin};
use super::ColorLibrary;

pub struct ColorSelectorPlugin;

impl Plugin for ColorSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SelectorInteractionPlugin)
            .add_systems(Startup, spawn_selector_buttons)
            .add_systems(Update, update_highlighted_ui);
    }
}

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
                    width: Val::Px(40.0),
                    height: Val::Px(40.0),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                ..default()
            },
            self.clone(),
        )
    }

    fn border_color(&self) -> Color {
        if self.is_highlighted {
            Color::WHITE
        } else {
            Color::DARK_GRAY
        }
    }
}

fn spawn_selector_buttons(mut commands: Commands, color_library: Res<ColorLibrary>) {
    commands
        .spawn((
            Name::new("Selector buttons container"),
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    column_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
        ))
        .with_children(|list| {
            let buttons = build_buttons_from_colors(color_library.all_colors());
            for node in build_buttons_ui(buttons, color_library.selected_color()) {
                list.spawn((Name::new("Selector button"), node));
            }
        });
}

fn update_highlighted_ui(
    mut buttons: Query<(&mut SelectorButton, &mut BorderColor)>,
    on_clicked: EventReader<OnColorClicked>,
    color_library: Res<ColorLibrary>,
) {
    // TODO: this function shouldn't rely on ColorLibrary, use value in OnColorClicked instead
    if !on_clicked.is_empty() {
        let buttons_to_update = buttons.iter_mut().filter(|(button, _)| {
            button.is_highlighted != check_if_selected(button, color_library.selected_color())
        });

        for (mut button, mut border) in buttons_to_update {
            button.is_highlighted = check_if_selected(&button, color_library.selected_color());
            border.0 = button.border_color();
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
            button.is_highlighted = check_if_selected(&button, selected_color);
            button.to_ui()
        })
        .collect()
}

fn check_if_selected(button: &SelectorButton, selected_color: Option<Color>) -> bool {
    Some(button.color) == selected_color
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // This aciton became an event so I don't think I can test it anymore?
    // #[test]
    // fn clicking_button_selects_that_color() {
    //     let mut library = ColorLibrary::empty();
    //     library.add_color(Color::RED);
    //     library.add_color(Color::GREEN);

    //     let selector = SelectorButton::new(Color::GREEN);

    //     assert_eq!(library.selected_color(), Some(Color::RED));

    //     selector.set_selected(&mut library);

    //     assert_eq!(library.selected_color(), Some(Color::GREEN));
    // }

    #[test]
    fn can_set_button_highlighted() {
        let mut selector = SelectorButton::new(Color::GREEN);

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
