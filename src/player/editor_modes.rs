pub mod build;
pub mod select;

use bevy::prelude::*;

use self::build::BuildModePlugin;

pub struct EditorModesPlugin;

impl Plugin for EditorModesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BuildModePlugin)
            .add_state::<EditorMode>()
            .add_systems(Update, switch_editor_mode);
    }
}

const MODE_SWITCH_KEY: KeyCode = KeyCode::M;

#[derive(Debug, PartialEq, Clone, Copy, States, Eq, Default, Hash)]
pub enum EditorMode {
    #[default]
    Build,
    Select,
}

fn switch_editor_mode(
    input: Res<Input<KeyCode>>,
    editor_mode: Res<State<EditorMode>>,
    mut next_editor_mode: ResMut<NextState<EditorMode>>,
) {
    if input.just_pressed(MODE_SWITCH_KEY) {
        next_editor_mode.set(get_next_editor_mode(*editor_mode.get()));
        println!("Switched to: {:?}", get_next_editor_mode(*editor_mode.get()));
    }
}

fn get_next_editor_mode(mode: EditorMode) -> EditorMode {
    match mode {
        EditorMode::Build => EditorMode::Select,
        EditorMode::Select => EditorMode::Build,
    }
}
