use crate::states::GameState;
use bevy::prelude::*;

mod controls_menu;
mod main_menu;

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    /// Called when the `App` registers the plugin to set the systems to run
    /// when the menus enter various states.
    ///
    /// # Arguments
    ///
    /// * `app` - The main Bevy app instance.
    fn build(&self, app: &mut AppBuilder) {
        // When the game state enters the `MainMenu` state, we build the main
        // menu.
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(main_menu::setup_main_menu.system()),
        )
        // When the game state goes back into the `MainMenu` state, we build
        // the main menu.
        .add_system_set(
            SystemSet::on_resume(GameState::MainMenu)
                .with_system(main_menu::setup_main_menu.system()),
        )
        // When the game updates, we set the interactions for the main menu
        // buttons.
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(main_menu::handle_menu_item_click.system()),
        )
        // When the game state is paused in the `MainMenu` state, we tear down
        // the main menu.
        .add_system_set(
            SystemSet::on_pause(GameState::MainMenu)
                .with_system(main_menu::teardown_menu_items.system()),
        )
        // When the game state exists the `MainMenu` state, we tear down the
        // main menu.
        .add_system_set(
            SystemSet::on_exit(GameState::MainMenu)
                .with_system(main_menu::teardown_menu_items.system()),
        );
        // When the game state enters the `ControlMenu` state, we build the
        // controls menu.
        app.add_system_set(
            SystemSet::on_enter(GameState::ControlMenu)
                .with_system(controls_menu::setup_controls_menu.system()),
        )
        // Add the interaction for the Back button in the controls menu.
        .add_system_set(
            SystemSet::on_update(GameState::ControlMenu)
                .with_system(controls_menu::handle_back_button_interaction.system()),
        )
        // When the game state exits the `ControlMenu` state, we tear down the
        // controls menu.
        .add_system_set(
            SystemSet::on_exit(GameState::ControlMenu)
                .with_system(controls_menu::teardown_controls_menu.system()),
        );
    }
}
