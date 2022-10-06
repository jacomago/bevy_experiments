use bevy::prelude::*;

use crate::{loading::FontAssets, systems::health::Health, GameState};

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

fn update_hud(
    mut health_change_event: EventReader<Health>,
    mut query: Query<(&mut Text, With<HealthText>)>,
) {
    let (mut text, _) = query.single_mut();
    for health_change in health_change_event.iter() {
        text.sections[0].value = format!("Health: {}", health_change.current);
    }
}
