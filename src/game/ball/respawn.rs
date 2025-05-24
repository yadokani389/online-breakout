use std::time::Duration;

use bevy::prelude::*;
use bevy_ggrs::prelude::*;

use crate::game::{components::Team, paddle::Paddle};

use super::{BALL_RADIUS, Ball, FIRST_BALL_SPEED, Velocity};

#[derive(Component, Clone)]
pub struct RespawningBall(Timer);

pub fn respawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_ball: Query<&Team, With<Ball>>,
) {
    let counts = q_ball.iter().fold([0; 2], |mut acc, team| {
        acc[team.0] += 1;
        acc
    });

    for (team, _) in counts
        .into_iter()
        .enumerate()
        .filter(|(_, count)| *count == 0)
    {
        commands
            .spawn((
                RespawningBall(Timer::new(Duration::from_secs(3), TimerMode::Once)),
                Ball {
                    radius: BALL_RADIUS,
                },
                Team(team),
                Mesh2d(meshes.add(Mesh::from(Circle::new(BALL_RADIUS)))),
                MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
                Transform::from_xyz(0., 10000., 10.),
                Velocity(Vec2::ZERO),
            ))
            .add_rollback();
    }
}

#[allow(clippy::type_complexity)]
pub fn handle_respawning_balls(
    mut commands: Commands,
    q_ball: Query<(
        Entity,
        &mut RespawningBall,
        &mut Transform,
        &mut Velocity,
        &Team,
    )>,
    q_paddle: Query<(&Team, &Transform), (With<Paddle>, Without<RespawningBall>)>,
    time: Res<Time>,
) {
    for (entity, mut timer, mut transform, mut velocity, team) in q_ball {
        timer.0.tick(time.delta());
        let Some(paddle_transform) = q_paddle
            .iter()
            .find(|(paddle_team, _)| paddle_team.0 == team.0)
            .map(|(_, transform)| transform)
        else {
            continue;
        };

        let relative_y = 1. - 2. * team.0 as f32;

        transform.translation.x = paddle_transform.translation.x;
        transform.translation.y = paddle_transform.translation.y + relative_y * 5. * BALL_RADIUS;

        if timer.0.finished() {
            velocity.0 = Vec2::new(0., relative_y).normalize() * FIRST_BALL_SPEED;
            commands.entity(entity).remove::<RespawningBall>();
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn despawn_stopped_balls(
    mut commands: Commands,
    q_ball: Query<(Entity, &Velocity), (With<Ball>, Without<RespawningBall>)>,
) {
    for (entity, velocity) in q_ball {
        if velocity.0.length_squared() < 0.01 {
            commands.entity(entity).despawn();
        }
    }
}
