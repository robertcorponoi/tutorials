use crate::states::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;

/// Represents the main menu of the game.
pub struct MainMenu;

/// The buttons in the main menu.
#[derive(Clone, Copy)]
pub enum MenuItem {
    /// The play button is used to start the game.
    Play,
    /// The controls button is used to open the controls menu.
    Controls,
    /// The exit button is used to exit the game.
    Exit,
}

/// Sets up the main menu by defining the layout, inserting the title text, and
/// spawning the buttons.
///
/// # Arguments
///
/// `commands` - Used to create the menu.
/// `asset_server` - Used to load our custom font.
/// `clear_color` - Used to create the solid background color for the main menu.
pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
) {
    // Load our custom font.
    let font: Handle<Font> = asset_server.load("fonts/RobotoMono-Regular.ttf");

    // Set the background color of the menu to black.
    clear_color.0 = Color::BLACK;

    commands
        // This is where we're going to define the layout of the main menu.
        .spawn_bundle(NodeBundle {
            style: Style {
                // We want the menu to take up 100% of the available width and
                // height. This means that on our 800x600 window, the menu will be
                // 800x600.
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                // Align the items in the main menu to the center both horizontally
                // and vertically.
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                // Use the default styles for everything else.
                ..Style::default()
            },
            visible: Visible {
                is_visible: false,
                ..Visible::default()
            },
            ..NodeBundle::default()
        })
        .insert(MainMenu)
        // Next, we add in the title and buttons for the main menu.
        .with_children(|mut parent| {
            // Starting with the title. We'll just set our title to be the same as
            // the game title but with a larger font, and white to stick out on the
            // black background.
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Bevy Simple Menu",
                    TextStyle {
                        font: font.clone(),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                    // Center the title both horizontally and vertically.
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            });

            // Our buttons to spawn. This will show as an error until we define the
            // function but we'll do it next.
            spawn_button(&mut parent, font.clone(), MenuItem::Play);
            spawn_button(&mut parent, font.clone(), MenuItem::Controls);
            spawn_button(&mut parent, font.clone(), MenuItem::Exit);
        });
}

/// Spawns a button within the main menu.
///
/// # Arguments
///
/// * `parent` - The parent which we can use to spawn the buttons with.
/// * `font` - The font to use for the button text.
/// * `item` - The `MenuItem` to spawn a button for.
fn spawn_button(parent: &mut ChildBuilder, font: Handle<Font>, menu_item: MenuItem) {
    // Create the container for the button. This is more or less the same
    // properties as the menu layout.
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                // The size of the button. We want a small button so we'll set
                // it to be 10% width of the screen and 30px high.
                size: Size {
                    width: Val::Percent(10.0),
                    height: Val::Px(30.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            ..ButtonBundle::default()
        })
        .insert(menu_item)
        // Next we'll create the text for the button depending on the `MenuItem`
        // that was provided.
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style::default(),
                text: Text::with_section(
                    // Set the text of the button depending on the `MenuItem` that
                    // it relates to.
                    match menu_item {
                        MenuItem::Play => "Play",
                        MenuItem::Controls => "Controls",
                        MenuItem::Exit => "Exit",
                    },
                    // If you decided to use a custom font you can pass it here
                    // and also define the background color of the button.
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::DARK_GRAY,
                    },
                    // Align the text in the center both vertically and
                    // horizontally within the button container.
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            });
        });
}

/// Handles what should when then a menu item in the main menu is selected
/// by the user.
///
/// # Arguments
///
/// * `app_exit_events` - The event writer that will send the signal that the app should exit.
/// * `app_state` - The current state of the game.
/// * `query` - The query for the buttons in the menu.
pub fn handle_menu_item_click(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    query: Query<(&Interaction, &MenuItem)>,
) {
    query.for_each(|(interaction, item)| match interaction {
        // Define what should happen when the buttons are clicked.
        Interaction::Clicked => match item {
            // When the play button is clicked, we push the `MainGame` state to
            // start the game.
            MenuItem::Play => {
                app_state
                    .push(GameState::MainGame)
                    .map_err(|err| error!("Failed to start game: {}", err))
                    .unwrap();
            }
            // When the controls button is clicked, we push the `ControlMenu`
            //  state to open the controls menu.
            MenuItem::Controls => {
                app_state
                    .push(GameState::ControlMenu)
                    .map_err(|err| error!("Failed to open control menu: {}", err))
                    .unwrap();
            }
            // When the exit button is clicked, we send the `AppExit` event to
            // exit the application.
            MenuItem::Exit => app_exit_events.send(AppExit),
        },
        // Optionally, if you're interesting in adding hover effects to the
        // buttons, you can do so here.
        Interaction::Hovered => {}
        _ => {}
    });
}

/// Tears down the main menu by removing all entities that are part of the
/// main menu.
///
/// # Arguments
///
/// * `commands` - The commands used to modify the `World`.
/// * `query` - The query to get the main menu and its entities.
pub fn teardown_menu_items(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
