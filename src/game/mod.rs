use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ggrs::prelude::*;
use components::Team;
use matchbox_socket::PeerId;

mod ball;
mod components;
mod field;
mod menu;
mod online;
mod paddle;

type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GgrsPlugin::<Config>::default(),
            menu::MenuPlugin,
            ball::BallPlugin,
            field::FieldPlugin,
            paddle::PaddlePlugin,
            online::OnlinePlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup_graphics)
        .rollback_component_with_clone::<Transform>()
        .rollback_component_with_copy::<Team>();
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 1100.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    Lobby,
    Matchmaking,
    InGame,
}
