use bevy::prelude::*;

pub mod ball;
pub mod field;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ball::BallPlugin, field::FieldPlugin))
            .add_systems(Startup, setup_graphics);
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2d);
}
