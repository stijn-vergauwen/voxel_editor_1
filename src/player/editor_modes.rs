pub mod build;
pub mod select;

use bevy::prelude::*;

use self::build::BuildModePlugin;

pub struct EditorModesPlugin;

impl Plugin for EditorModesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BuildModePlugin)
            .insert_resource(CurrentEditorMode {
                mode: EditorMode::Build,
            })
            .add_systems(Update, switch_editor_mode);
    }
}

const MODE_SWITCH_KEY: KeyCode = KeyCode::M;

#[derive(Resource, Debug)]
pub struct CurrentEditorMode {
    mode: EditorMode,
}

impl CurrentEditorMode {
    pub fn mode(&self) -> EditorMode {
        self.mode
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EditorMode {
    Build,
    Select,
}

fn switch_editor_mode(input: Res<Input<KeyCode>>, mut editor_mode: ResMut<CurrentEditorMode>) {
    if input.just_pressed(MODE_SWITCH_KEY) {
        editor_mode.mode = get_next_editor_mode(editor_mode.mode);
    }
}

fn get_next_editor_mode(mode: EditorMode) -> EditorMode {
    match mode {
        EditorMode::Build => EditorMode::Select,
        EditorMode::Select => EditorMode::Build,
    }
}
