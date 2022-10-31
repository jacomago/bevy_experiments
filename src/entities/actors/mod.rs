use bevy::prelude::*;

mod monsters;
mod npc;
mod player;

pub use monsters::Monster;
pub use npc::AvailableQuest;
pub use npc::Npc;
pub use player::MapLevel;
pub use player::Player;

use self::monsters::MonstersPlugin;
use self::npc::NPCsPlugin;
use self::player::PlayerPlugin;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(MonstersPlugin)
            .add_plugin(NPCsPlugin);
    }
}
