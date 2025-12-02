use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::game::{
    GameState,
    online::{IrohId, network_role::NetworkRole},
};

pub struct MatchmakingPlugin;

impl Plugin for MatchmakingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            show_text.run_if(in_state(GameState::Matchmaking)),
        );
    }
}

fn show_text(mut context: EguiContexts, iroh_id: Option<Res<IrohId>>, role: Res<NetworkRole>) {
    let message = if matches!(*role, NetworkRole::Host) {
        if let Some(id) = iroh_id.map(|id| id.0.to_string()) {
            format!(
                "Share this ID\n{0}\nor open this\nhttps://yadokani389.github.io/online-breakout/#{0}",
                id
            )
        } else {
            "Connecting...".into()
        }
    } else {
        "Connecting...".into()
    };

    egui::CentralPanel::default().show(context.ctx_mut().unwrap(), |ui| {
        ui.horizontal_centered(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 40.),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    for line in message.lines() {
                        ui.label(egui::RichText::new(line).size(20.));
                    }
                },
            );
        });
    });
}
