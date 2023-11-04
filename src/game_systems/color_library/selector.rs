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
    mut on_clicked: EventReader<OnColorClicked>,
) {
    for event in on_clicked.iter() {
        for (mut button, mut border) in buttons.iter_mut() {
            update_button_highlight(&mut button, &mut border, event.color);
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

fn update_button_highlight(
    button: &mut SelectorButton,
    border: &mut BorderColor,
    selected_color: Color,
) {
    let has_selected_color = button.color == selected_color;

    if button.is_highlighted != has_selected_color {
        button.is_highlighted = has_selected_color;
        border.0 = button.border_color();
    }
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

    #[test]
    fn correctly_updates_button_highlights_when_color_changed() {
        let mut buttons = build_buttons_from_colors(vec![Color::RED, Color::GREEN, Color::BLUE]);
        buttons[0].is_highlighted = true;

        assert_eq!(find_selected_color(&buttons), Some(Color::RED));
        assert_eq!(count_highlighted_buttons(&buttons), 1);

        for button in buttons.iter_mut() {
            update_button_highlight(button, &mut BorderColor(Color::WHITE), Color::GREEN);
        }

        assert_eq!(find_selected_color(&buttons), Some(Color::GREEN));
        assert_eq!(count_highlighted_buttons(&buttons), 1);

        fn find_selected_color(buttons: &Vec<SelectorButton>) -> Option<Color> {
            buttons
                .iter()
                .find(|button| button.is_highlighted)
                .map(|button| button.color)
        }

        fn count_highlighted_buttons(buttons: &Vec<SelectorButton>) -> usize {
            buttons
                .iter()
                .filter(|button| button.is_highlighted)
                .count()
        }
    }
}
