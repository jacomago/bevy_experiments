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
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Px(10.),
                    right: Val::Px(10.),
                    ..default()
                },
                ..default()
            },
            color: UiColor(Color::NONE),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("Health: {}", health),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.6, 0.6, 0.6),
                                font: font.fira_sans.clone(),
                            },
                        }],
                        alignment: default(),
                    },
                    ..default()
                })
                .insert(HealthText);
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
