use bevy::prelude::*;

use self::{hud::HUDPlugin, tooltip::TooltipPlugin};

mod health_bar;
mod hud;
mod inventory;
mod quests;
pub mod tooltip;

pub use hud::HudComponent;
pub use tooltip::ToolTip;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(HUDPlugin).add_plugin(TooltipPlugin);
    }
}
