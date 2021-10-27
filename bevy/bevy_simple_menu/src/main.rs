use bevy::{prelude::*, window::WindowMode};

mod camera;
mod menus;
mod states;

fn main() {
    let mut app = App::build();

    // Set the game to:
    // - be windowed
    // - have a resolution of 800x600
    // - no vsync
    // - not be able to be resized
    // - set the rest of the settings to their default values
    app.insert_resource(WindowDescriptor {
        width: 800.0,
        height: 600.0,
        title: "Bevy Simple Menu".to_string(),
        vsync: false,
        mode: WindowMode::Windowed,
        resizable: false,
        ..Default::default()
    });

    // Add the plugins we need.
    app.add_plugins(DefaultPlugins)
        .add_plugin(menus::MenusPlugin);

    // Add the camera as a startup system.
    app.add_startup_system(camera::spawn_ui_camera.system());

    // Add the starting state. We want the user to start at the main menu.
    app.add_state(states::GameState::MainMenu);

    // Start the game.
    app.run();
}
