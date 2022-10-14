use bevy::prelude::*;

use crate::actors::{components::health::Health, Player};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WantsToAttack>();
    }
}

pub fn combat(
    mut commands: Commands,
    mut combat_events: EventReader<WantsToAttack>,
    player_query: Query<(Entity, With<Player>)>,
    mut healths: Query<&mut Health>,
) {
    let (player, _) = player_query.single();
    combat_events.iter().for_each(|event| {
        if let Ok(mut health) = healths.get_mut(event.victim) {
            const COMBAT_STRENGTH: i32 = 1;

            info!("Attack, health start: {}", health.current);
            health.current -= COMBAT_STRENGTH;
            if health.current < 1 && event.victim != player {
                commands.entity(event.victim).despawn_recursive();
            }
            info!("Attack, health end: {}", health.current);
        }
    });
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
