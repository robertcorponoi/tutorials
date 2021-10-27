use crate::states::GameState;
use bevy::prelude::*;

/// Represents the menu that displays the controls of the game.
pub struct ControlMenu;

/// Represents the button that returns the player to the main menu.
pub struct BackButton;

/// Sets up the controls menu by defining the layout, inserting the title text, and
/// spawning the Back button.
///
/// # Arguments
///
/// * `commands` - A list of commands that will be run to modify a `World`.
/// * `asset_server` - Used to load assets from the filesystem on background threads.
pub fn setup_controls_menu(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // Load our custom font.
    let font: Handle<Font> = asset_server.load("fonts/RobotoMono-Regular.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            visible: Visible {
                is_visible: false,
                ..Visible::default()
            },
            ..NodeBundle::default()
        })
        .insert(ControlMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style { ..Style::default() },
                text: Text::with_section(
                    "Use \"WASD\" to move and \"E\" to interact with things.",
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size {
                            width: Val::Percent(10.),
                            height: Val::Px(30.),
                        },
                        flex_direction: FlexDirection::ColumnReverse,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..Style::default()
                    },
                    ..ButtonBundle::default()
                })
                // Adds the "Back" button to return the user to the game.
                .insert(BackButton)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style::default(),
                        text: Text::with_section(
                            "Back",
                            TextStyle {
                                font,
                                font_size: 20.0,
                                color: Color::DARK_GRAY,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        ..TextBundle::default()
                    });
                });
        });
}

/// Tears down the controls menu by removing all entities that are part of the
/// controls menu.
///
/// # Arguments
///
/// * `commands` - The commands used to modify the `World`.
/// * `query` - The controls menu query.
pub fn teardown_controls_menu(mut commands: Commands, query: Query<Entity, With<ControlMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// When the Back button is clicked we pop the `ControlsMenu` state so that the
/// game goes back to the `MainMenu` state.
///
/// # Arguments
///
/// * `app_state` - The state of the app.
/// * `query` - The query for the back button.
pub fn handle_back_button_interaction(
    mut app_state: ResMut<State<GameState>>,
    query: Query<&Interaction, With<BackButton>>,
) {
    query.for_each(|interaction| match interaction {
        // When the back button is clicked, we return the user to their
        // previous state.
        Interaction::Clicked => {
            #[cfg(debug_assertions)]
            info!("Popped game state");

            app_state
                .pop()
                .map_err(|err| error!("Failed to return to main menu: {}", err))
                .unwrap();
        }

        // Hover effects can be applied here.
        Interaction::Hovered => {}
        // Catch all for interactions.
        Interaction::None => {}
    });
}
