use bevy::prelude::*;

pub struct MouseInteractionPlugin;

impl Plugin for MouseInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseInteraction::default())
            .add_systems(Update, update_mouse_on_ui);
    }
}

#[derive(Resource)]
pub struct MouseInteraction {
    active_camera: Option<Entity>,
    mouse_on_ui: bool,
}

impl Default for MouseInteraction {
    fn default() -> Self {
        Self {
            active_camera: None,
            mouse_on_ui: false,
        }
    }
}

impl MouseInteraction {
    pub fn set_active_camera(&mut self, camera_entity: Entity) {
        self.active_camera = Some(camera_entity);
    }
}

fn update_mouse_on_ui(mut mouse_interaction: ResMut<MouseInteraction>, nodes: Query<&Interaction>) {
    mouse_interaction.mouse_on_ui = nodes.iter().any(|interaction| {
        *interaction == Interaction::Hovered || *interaction == Interaction::Pressed
    });
}
