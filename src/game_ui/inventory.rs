use bevy::prelude::*;
use winit::dpi::Position;

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
                        top: Val::Auto,
                        ..default()
                    },
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
                .insert(Inventory);
        });
}

#[derive(Component)]
pub struct Inventory;

pub fn update_inventory(
    mut commands: Commands,
    player_items: Query<(With<Carried>, &CharacterName)>,
    inventory_query: Query<(Entity, With<Inventory>)>,
    font: Res<FontAssets>,
) {
    let (inventory, _) = inventory_query.single();
    commands.entity(inventory).despawn_descendants();
    commands.entity(inventory).with_children(|parent| {
        player_items.iter().for_each(|(_, name)| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    name.0.clone(),
                    TextStyle {
                        font: font.fira_sans.clone(),
                        font_size: 10.,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    size: Size::new(Val::Undefined, Val::Px(25.)),
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Auto,
                        ..default()
                    },
                    ..default()
                }),
            );
        });
    });
}
