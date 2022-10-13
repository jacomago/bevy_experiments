use bevy::prelude::*;

use crate::{
    actors::{components::health::Health, Player},
    loading::FontAssets,
    GameState,
};

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_hud))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_hud));
    }
}

fn setup_hud(mut commands: Commands, font: Res<FontAssets>) {
    let health = 20.0;
    let health_percentage = 100.0;
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(10.)),
                position: UiRect {
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    ..default()
                },
                ..default()
            },
            color: UiColor(Color::rgba(0.65, 0.65, 0.65, 0.5)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(health_percentage), Val::Px(10.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: UiColor(Color::rgba(0.0, 0.65, 0.0, 0.5)),
                    ..default()
                })
                .insert(HealthBar)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: format!("Health: {}", health),
                                    style: TextStyle {
                                        font_size: 10.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        font: font.fira_sans.clone(),
                                    },
                                }],
                                alignment: TextAlignment::CENTER,
                            },
                            ..default()
                        })
                        .insert(HealthText);
                });
        });
}

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct HealthBar;

fn calc_health_percentage(health: Health) -> f32 {
    (100 * health.current / health.max) as f32
}

fn update_hud(
    player_health: Query<(&Health, With<Player>)>,
    mut text_query: Query<(&mut Text, With<HealthText>)>,
    mut bar_query: Query<(&mut Style, With<HealthBar>)>,
) {
    let (health, _) = player_health.single();
    let (mut text, _) = text_query.single_mut();
    let new_text_string = format!("Health: {}", health.current);
    if new_text_string != text.sections[0].value {
        let (mut bar_style, _) = bar_query.single_mut();
        text.sections[0].value = new_text_string;
        bar_style.size = Size::new(Val::Percent(calc_health_percentage(*health)), Val::Px(10.));
    }
}
