use bevy::{prelude::*, render::camera::ScalingMode};

pub mod ball;
pub mod field;
pub mod paddle;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ball::BallPlugin, field::FieldPlugin, paddle::PaddlePlugin))
            .add_systems(Startup, setup_graphics);
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
pub struct Team(pub u8);
