use bevy::prelude::*;

use crate::{
    components::{carried::Carried, name::CharacterName},
    loading::FontAssets,
};

use super::hud::Hud;

pub fn spawn_inventory(mut commands: Commands, font: Res<FontAssets>) {
    // Inventory
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                ..default()
            },
            color: UiColor(Color::rgba(0.65, 0.65, 0.65, 0.5)),
            ..default()
        })
        .insert(Hud)
        .with_children(|parent| {
            // Title
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Inventory",
                    TextStyle {
                        font: font.fira_sans.clone(),
                        font_size: 16.,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    size: Size::new(Val::Undefined, Val::Px(25.)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        ..default()
                    },
                    ..default()
                }),
            );
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![],
                        alignment: TextAlignment::CENTER,
                    },
                    ..default()
                })
                .insert(Inventory);
        });
}

#[derive(Component)]
pub struct Inventory;

pub fn update_inventory(
    player_items: Query<(With<Carried>, &CharacterName)>,
    mut text_query: Query<(&mut Text, With<Inventory>)>,
    font: Res<FontAssets>,
) {
    let (mut text, _) = text_query.single_mut();
    text.sections = player_items
        .iter()
        .map(|(_, name)| {
            info!("Adding {}", name.0);
            TextSection {
                value: name.0.clone(),
                style: TextStyle {
                    font: font.fira_sans.clone(),
                    font_size: 25.,
                    color: Color::BLACK,
                },
            }
        })
        .collect();
}
