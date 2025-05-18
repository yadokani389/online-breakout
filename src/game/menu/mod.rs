use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod lobby;
pub mod matchmaking;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            lobby::LobbyPlugin,
            matchmaking::MatchmakingPlugin,
        ));
    }
}
