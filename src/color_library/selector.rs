use bevy::prelude::*;

use super::ColorLibrary;

pub struct ColorSelectorPlugin;

impl Plugin for ColorSelectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_selector_buttons)
            .add_systems(Update, select_next_color_on_key);
    }
}

const NEXT_COLOR_KEY: KeyCode = KeyCode::Period;

struct SelectorButton {
    color: Color,
}

impl SelectorButton {
    fn new(color: Color) -> Self {
        Self { color }
    }

    fn to_ui(&self) -> NodeBundle {
        NodeBundle {
            background_color: self.color.into(),
            style: Style {
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                ..default()
            },
            ..default()
        }
    }
}

fn select_next_color_on_key(input: Res<Input<KeyCode>>, mut color_library: ResMut<ColorLibrary>) {
    if input.just_pressed(NEXT_COLOR_KEY) {
        color_library.select_next();

        println!(
            "Color '{:?}' is now selected.",
            color_library.selected_color()
        );
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
            for color in color_library.all_colors() {
                let selector_button = SelectorButton::new(color);

                list.spawn(selector_button.to_ui());
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: clicking on button selects that color
    // TODO: highlight button that has selected color
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

        let node: NodeBundle = selector.to_ui();

        assert_eq!(node.background_color.0, Color::GREEN);
    }
}
