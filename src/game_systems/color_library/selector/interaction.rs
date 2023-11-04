use bevy::prelude::*;

use crate::game_systems::color_library::OnColorClicked;

use super::SelectorButton;

pub struct SelectorInteractionPlugin;

impl Plugin for SelectorInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_interaction);
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
