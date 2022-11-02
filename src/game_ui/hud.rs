use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::GameState;

use super::{
    health_bar::update_hud_health, inventory::update_inventory_hud, quests::update_quests_hud,
};

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(hud_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(hud_update)
                    .with_system(update_hud_health)
                    .with_system(update_inventory_hud)
                    .with_system(update_quests_hud),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(hud_cleanup));
    }
}

const LEFT_PANEL: &str = "left panel";
const TOP_PANEL: &str = "top panel";
const RIGHT_PANEL: &str = "right panel";

fn hud_cleanup(mut egui_context: ResMut<EguiContext>) {
    let ctx = egui_context.ctx_mut();
    egui::SidePanel::left(LEFT_PANEL).show(ctx, |_| {});
    egui::SidePanel::right(RIGHT_PANEL).show(ctx, |_| {});
    egui::TopBottomPanel::top(TOP_PANEL).show(ctx, |_| {});
}

#[derive(Debug, Default)]
pub struct QuestsStrings {
    pub assigned: Vec<String>,
    pub updated: Vec<String>,
    pub completed: Vec<String>,
}

#[derive(Debug, Default)]
pub struct UiState {
    pub player_health_percentage: f32,
    pub quests: QuestsStrings,
    pub inventory: Vec<String>,
}

fn hud_setup(mut commands: Commands, mut egui_context: ResMut<EguiContext>) {
    let mut visuals = egui::Visuals::dark();
    visuals.widgets.noninteractive.bg_fill = egui::color::Color32::TRANSPARENT;
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::none();
    visuals.widgets.noninteractive.fg_stroke.color = egui::color::Color32::WHITE;
    let style = egui::Style {
        visuals,
        ..default()
    };
    let ctx = egui_context.ctx_mut();
    ctx.set_style(style);
    
    let ui_status = UiState::default();
    commands.insert_resource(ui_status);
}

fn hud_update(mut egui_context: ResMut<EguiContext>, ui_status: Res<UiState>) {
    if !ui_status.is_changed() {
        return;
    }
    let ctx = egui_context.ctx_mut();
    egui::SidePanel::left(LEFT_PANEL).show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.heading("Quests");
            ui.end_row();

            ui.separator();
            ui_status.quests.assigned.iter().for_each(|s| {
                ui.colored_label(egui::Color32::WHITE, s);
            });

            ui.separator();
            ui_status.quests.updated.iter().for_each(|s| {
                ui.colored_label(egui::Color32::LIGHT_YELLOW, s);
            });
            ui.separator();
            ui_status.quests.completed.iter().for_each(|s| {
                ui.colored_label(egui::Color32::LIGHT_GREEN, s);
            });
        });
    });
    egui::SidePanel::right(RIGHT_PANEL).show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.heading("Inventory");

            ui.separator();
            ui.separator();
            ui_status.inventory.iter().for_each(|s| {
                ui.label(s);
            });
        });
    });
    egui::TopBottomPanel::top(TOP_PANEL).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.visuals_mut().selection.bg_fill = egui::color::Color32::DARK_GREEN;
            let progress_bar = egui::ProgressBar::new(ui_status.player_health_percentage)
                .show_percentage()
                .text(format!("Health: {}", ui_status.player_health_percentage));
            ui.add(progress_bar);
        });
    });
}
