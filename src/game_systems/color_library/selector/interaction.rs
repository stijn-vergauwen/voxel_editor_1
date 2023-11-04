use bevy::prelude::*;

use crate::game_systems::color_library::ColorLibrary;

use super::SelectorButton;

pub struct SelectorInteractionPlugin;

impl Plugin for SelectorInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnColorClicked>()
            .add_systems(Update, (select_next_color_on_key, button_interaction));
    }
}

const NEXT_COLOR_KEY: KeyCode = KeyCode::Period;

#[derive(Event)]
pub struct OnColorClicked {
    pub color: Color,
}

impl OnColorClicked {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

fn select_next_color_on_key(input: Res<Input<KeyCode>>, mut color_library: ResMut<ColorLibrary>) {
    if input.just_pressed(NEXT_COLOR_KEY) {
        color_library.select_next();
    }
}

fn button_interaction(
    buttons: Query<(&SelectorButton, &Interaction), Changed<Interaction>>,
    mut on_clicked: EventWriter<OnColorClicked>,
) {
    for (button, interaction) in buttons.iter() {
        if *interaction == Interaction::Pressed {
            on_clicked.send(OnColorClicked::new(button.color));
        }
    }
}
