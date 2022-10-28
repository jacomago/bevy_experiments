use bevy::prelude::*;

use crate::{
    components::{damage::Damage, health::Health},
    entities::{Player, Weapon},
};

use super::inventory::Carried;

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
    damages: Query<&Damage>,
    weapons: Query<&Carried, With<Weapon>>,
) {
    let (player, _) = player_query.single();
    combat_events.iter().for_each(|event| {
        let final_damage = calc_damage(&damages, event, &weapons);

        if let Ok(mut health) = healths.get_mut(event.victim) {
            info!("Attack, health start: {}", health.current);
            health.current -= final_damage;
            if health.current < 1 && event.victim != player {
                commands.entity(event.victim).despawn_recursive();
            }
            info!("Attack, health end: {}", health.current);
        }
    });
}

fn calc_damage(
    damages: &Query<&Damage>,
    event: &WantsToAttack,
    weapons: &Query<&Carried, With<Weapon>>,
) -> i32 {
    let base_damage = if let Ok(damage) = damages.get(event.attacker) {
        damage.0
    } else {
        0
    };
    let weapon_damage: i32 = weapons
        .iter()
        .filter(|w| w.entity == event.attacker)
        .map(|w| damages.get(w.entity).map(|d| d.0).unwrap_or(0))
        .sum();
    base_damage + weapon_damage
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
