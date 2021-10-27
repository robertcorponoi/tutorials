use bevy::prelude::{Commands, UiCameraBundle};

/// Creates the default camera for the game.
///
/// # Arguments
///
/// * `commands` - A list of commands used to modify a `World`.
pub fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
