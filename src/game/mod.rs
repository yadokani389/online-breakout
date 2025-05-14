use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ggrs::prelude::*;
use matchbox_socket::PeerId;

pub mod ball;
pub mod field;
pub mod online;
pub mod paddle;

type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GgrsPlugin::<Config>::default(),
            ball::BallPlugin,
            field::FieldPlugin,
            paddle::PaddlePlugin,
            online::OnlinePlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup_graphics)
        .rollback_component_with_clone::<Transform>();
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

#[derive(Component, Deref, DerefMut, PartialEq)]
pub struct Team(pub usize);

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    Matchmaking,
    InGame,
}
