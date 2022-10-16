use crate::loading::FontAssets;
use crate::GameState;
use bevy::prelude::*;

/// Starting message displayed in the Menu
static WELCOME_MESSAGE: &str = "Welcome to the dungeon!";
/// Message displayed if the game is lost
pub static LOST_MESSAGE: &str = "You lost :( Try again?";

/// Pluging for the Menu for starting new games
pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .init_resource::<PlayerMessage>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(click_play_button))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu));
    }
}

/// Message displayed in the menu
pub struct PlayerMessage {
    /// The main message
    pub message: String,
}

impl Default for PlayerMessage {
    fn default() -> Self {
        Self {
            message: WELCOME_MESSAGE.to_owned(),
        }
    }
}

/// Colors of the button
struct ButtonColors {
    /// Color with no hover
    normal: UiColor,
    /// color with hover
    hovered: UiColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
        }
    }
}

/// Component that has the full menu
#[derive(Component)]
struct Menu;

/// Set up the menu/ spawn into the game
fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    message: Res<PlayerMessage>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(100.0)),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: message.message.clone(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            font: font_assets.fira_sans.clone(),
                        },
                    }],
                    alignment: TextAlignment::CENTER,
                },
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            });
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Undefined, Val::Px(50.0)),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: button_colors.normal,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Play".to_string(),
                                style: TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            }],
                            alignment: Default::default(),
                        },
                        ..Default::default()
                    });
                });
        });
}

/// Action after clicking the play button
fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    interaction_query
        .iter_mut()
        .for_each(|(interaction, mut color)| match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *color = button_colors.hovered;
            }
            Interaction::None => {
                *color = button_colors.normal;
            }
        });
}

/// Remove the menu from the app after started playing

fn cleanup_menu(
    mut commands: Commands,
    menu: Query<Entity, With<Menu>>,
    camera: Query<Entity, With<Camera2d>>,
) {
    commands.entity(menu.single()).despawn_recursive();
    commands.entity(camera.single()).despawn_recursive();
}
