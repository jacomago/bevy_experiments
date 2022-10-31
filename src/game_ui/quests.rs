use bevy::prelude::*;

use crate::{
    components::name::EntityName,
    loading::FontAssets,
    systems::quest_engine::{AssignedQuest, PlayerQuests},
};

use super::hud::HudComponent;

pub fn spawn_quests_ui(mut commands: Commands, font: Res<FontAssets>) {
    // Quests
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Auto, Val::Px(50.0)),
                ..default()
            },
            color: UiColor(Color::rgba(0.65, 0.65, 0.65, 0.5)),
            ..default()
        })
        .insert(HudComponent)
        .with_children(|parent| {
            // Title
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Quests",
                    TextStyle {
                        font: font.fira_sans.clone(),
                        font_size: 14.,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    size: Size::new(Val::Undefined, Val::Px(25.)),
                    ..default()
                }),
            );
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    color: UiColor(Color::rgba(0.65, 0.65, 0.65, 0.5)),
                    ..default()
                })
                .insert(HUDQuests);
        });
}

#[derive(Component)]
pub struct HUDQuests;

pub fn update_quests_hud(
    mut commands: Commands,
    mut quests_query: Query<&mut PlayerQuests>,
    assigned_quests: Query<(With<AssignedQuest>, &EntityName)>,
    hud_quests_query: Query<(Entity, With<HUDQuests>)>,
    font: Res<FontAssets>,
) {
    let mut quests = quests_query.single_mut();
    if !quests.is_dirty {
        return;
    }

    let (hud_quests, _) = hud_quests_query.single();
    // Remove old quests from ui.
    commands.entity(hud_quests).despawn_descendants();
    // Add updated one.
    commands.entity(hud_quests).with_children(|parent| {
        quests.key_map.iter().enumerate().for_each(|(i, entity)| {
            let (_, name) = assigned_quests.get(*entity).unwrap();
            parent.spawn_bundle(
                TextBundle::from_section(
                    format!("{}: {}", i, name),
                    TextStyle {
                        font: font.fira_sans.clone(),
                        font_size: 10.,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    size: Size::new(Val::Undefined, Val::Px(25.)),
                    ..default()
                }),
            );
        });
    });
    quests.is_dirty = false;
}
