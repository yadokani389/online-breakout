use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ggrs::prelude::*;
use components::Team;
use matchbox_socket::PeerId;

mod ball;
mod components;
mod field;
mod item;
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
            item::ItemPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup_graphics)
        .add_systems(
            GgrsSchedule,
            despawn_out_of_bounds_entities.after(field::toggle_cell),
        )
        .rollback_component_with_clone::<Transform>()
        .rollback_component_with_copy::<Team>();
    }
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    Lobby,
    Matchmaking,
    InGame,
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

fn despawn_out_of_bounds_entities(mut commands: Commands, query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query {
        if 1200. < transform.translation.x.abs() || 1200. < transform.translation.y.abs() {
            commands.entity(entity).despawn();
        }
    }
}
