use bevy::prelude::*;

use crate::{components::health::Health, entities::Player};

use super::hud::UiState;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct HealthBar;

fn calc_health_percentage(health: Health) -> f32 {
    (100 * health.current / health.max) as f32
}

pub fn update_hud_health(
    player_health: Query<(&Health, With<Player>)>,
    mut ui_status: ResMut<UiState>,
) {
    let (health, _) = player_health.single();
    ui_status.player_health_percentage = calc_health_percentage(*health);
}
