use bevy::prelude::*;

use crate::{
    components::name::EntityName,
    loading::FontAssets,
    systems::inventory::{Carried, PlayerInventory},
};

use super::hud::HudComponent;

pub fn spawn_inventory(mut commands: Commands, font: Res<FontAssets>) {
    // Inventory
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
                    "Inventory",
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
                .insert(HUDInventory);
        });
}

#[derive(Component)]
pub struct HUDInventory;

pub fn update_inventory_hud(
    mut commands: Commands,
    mut inventory_query: Query<&mut PlayerInventory>,
    items: Query<(With<Carried>, &EntityName)>,
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
        inventory
            .key_map
            .iter()
            .enumerate()
            .for_each(|(i, entity)| {
                let (_, name) = items.get(*entity).unwrap();
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
    inventory.is_dirty = false;
}
