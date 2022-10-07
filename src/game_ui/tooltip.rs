use bevy::prelude::*;

use crate::{actions::Actions, loading::FontAssets, map::map_position::MapPosition, GameState};

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToolTipInfo>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_tooltip))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(mouse_rollover)
                    .with_system(update_tooltip),
            );
    }
}

#[derive(Component, Default)]
pub struct Interactive {
    text: String,
}

#[derive(Component)]
struct ToolTip;

struct ToolTipInfo {
    text: Option<String>,
    position: Option<Vec2>,
}

fn mouse_rollover(
    actions: Res<Actions>,
    interactives: Query<(&MapPosition, &Interactive)>,
    mut tooltip_event: EventWriter<ToolTipInfo>,
) {
    if actions.mouse_rollover.is_none() {
        return;
    }
    let mut overlap = false;
    let mouse_position = actions.mouse_rollover.as_ref().unwrap();
    for (map_position, interactive) in interactives.iter() {
        if map_position.overlaps(mouse_position.game_position) {
            tooltip_event.send(ToolTipInfo {
                text: Some(interactive.text.clone()),
                position: Some(mouse_position.screen_position),
            });
            overlap = true;
        }
    }
    if !overlap {
        tooltip_event.send(ToolTipInfo {
            text: None,
            position: None,
        })
    }
}

fn spawn_tooltip(mut commands: Commands, font: Res<FontAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(10.0), Val::Px(10.)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    ..default()
                },
                ..default()
            },
            color: UiColor(Color::rgba(0.65, 0.65, 0.65, 0.5)),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(),
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
                .insert(ToolTip);
        });
}

fn update_tooltip(
    mut current_tooltip: Query<(&mut Visibility, &mut Style, &mut Text, With<ToolTip>)>,
    mut tooltip_info: EventReader<ToolTipInfo>,
) {
    let (mut visibility, mut style, mut text, _) = current_tooltip.single_mut();
    for poss_info in tooltip_info.iter() {
        match &poss_info.text {
            Some(info_text) => {
                text.sections[0].value = info_text.clone();
                visibility.is_visible = true;
            }
            None => {
                visibility.is_visible = false;
            }
        }
        match &poss_info.position {
            Some(position) => {
                style.position = UiRect {
                    left: Val::Px(position.x),
                    bottom: Val::Px(position.y),
                    ..default()
                };
            }
            None => {}
        }
    }
}
