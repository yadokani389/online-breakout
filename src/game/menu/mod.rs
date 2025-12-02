use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod lobby;
pub mod matchmaking;
pub mod result;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            lobby::LobbyPlugin,
            matchmaking::MatchmakingPlugin,
            result::ResultPlugin,
        ));
    }
}
