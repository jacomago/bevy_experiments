use bevy::{prelude::*, utils::HashMap};

use crate::{
    components::{carried::Carried, name::EntityName},
    entities::Player,
    loading::FontAssets,
};

use super::hud::HudComponent;

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
        .insert(HudComponent)
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
                .insert(HUDInventory);
        });
}

#[derive(Component)]
pub struct HUDInventory;

pub fn update_inventory_hud(
    mut commands: Commands,
    mut inventory_query: Query<&mut PlayerInventory>,
    hud_inventory_query: Query<(Entity, With<HUDInventory>)>,
    font: Res<FontAssets>,
) {
    let mut inventory = inventory_query.single_mut();
    if !inventory.is_dirty {
        return;
    }

    let (hud_inventory, _) = hud_inventory_query.single();
    // Remove old inventory from ui.
    commands.entity(hud_inventory).despawn_descendants();
    // Add updated one.
    commands.entity(hud_inventory).with_children(|parent| {
        inventory.key_map.iter().enumerate().for_each(|(i, name)| {
            let count = inventory.counts.get(name).unwrap();
            parent.spawn_bundle(
                TextBundle::from_section(
                    format!("{} {}: {}", i, name, count),
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
                        top: Val::Auto,
                        ..default()
                    },
                    ..default()
                }),
            );
        });
    });
    inventory.is_dirty = false;
}

#[derive(Component, Default)]
pub struct PlayerInventory {
    key_map: Vec<String>,
    counts: HashMap<String, u32>,
    is_dirty: bool,
}

pub fn update_inventory(
    player_query: Query<(Entity, With<Player>)>,
    player_items: Query<(&Carried, &EntityName)>,
    mut inventory_query: Query<&mut PlayerInventory>,
) {
    let mut new_inventory = HashMap::new();
    let (player, _) = player_query.single();
    player_items
        .iter()
        .filter(|(c, _)| c.entity == player)
        .for_each(|(_, i)| {
            let current = new_inventory.entry(i.0.clone()).or_insert(0);
            *current += 1;
        });
    let mut inventory = inventory_query.single_mut();
    if new_inventory != inventory.counts {
        inventory.is_dirty = true;

        let mut keys = new_inventory.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        inventory.key_map = keys;
        inventory.counts = new_inventory;
    }
}
