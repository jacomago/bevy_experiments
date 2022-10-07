use bevy::prelude::*;

use self::{hud::HUDPlugin, tooltip::TooltipPlugin};

mod hud;
pub mod tooltip;


pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HUDPlugin)
            .add_plugin(TooltipPlugin);
    }
}
