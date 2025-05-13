use crate::game::field::Cell;
use bevy::{math::bounding::Aabb2d, prelude::*};
use bevy_ggrs::prelude::*;

use super::Team;
use super::field::{CellClicked, Wall};
use super::paddle::{Paddle, move_paddles};

const FIRST_BALL_SPEED: f32 = 300.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ball).add_systems(
            GgrsSchedule,
            (apply_velocity.after(move_paddles), check_collision).chain(),
        );
    }
}

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 10.0;
    let initial_velocity = Vec2::new(1.0, 1.0).normalize() * FIRST_BALL_SPEED;
    commands
        .spawn((
            Ball { radius },
            Team(0),
            Mesh2d(meshes.add(Mesh::from(Circle::new(radius)))),
            MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
            Transform::from_xyz(0., -300., 10.),
            Velocity(initial_velocity),
        ))
        .add_rollback();
    commands
        .spawn((
            Ball { radius },
            Team(1),
            Mesh2d(meshes.add(Mesh::from(Circle::new(radius)))),
            MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
            Transform::from_xyz(0., 300., 10.),
            Velocity(initial_velocity),
        ))
        .add_rollback();
}

fn apply_velocity(time: Res<Time>, mut q_ball: Query<(&Velocity, &mut Transform), With<Ball>>) {
    for (velocity, mut transform) in q_ball.iter_mut() {
        transform.translation += velocity.extend(0.) * time.delta_secs();
    }
}

fn check_collision(
    mut commands: Commands,
    mut q_ball: Query<(Entity, &Ball, &Team, &mut Transform, &mut Velocity)>,
    q_cell: Query<(Entity, &Cell, &Team, &Transform), Without<Ball>>,
    q_wall: Query<(&Wall, &Transform), Without<Ball>>,
    q_paddle: Query<(&Paddle, &Team, &Transform), Without<Ball>>,
    mut events: EventWriter<CellClicked>,
) {
    'ball: for (ball_entity, ball, ball_team, mut ball_transform, mut velocity) in q_ball.iter_mut()
    {
        let ball_pos = ball_transform.translation.truncate();

        // Check for wall collisions
        for (wall, wall_transform) in q_wall.iter() {
            let closest_point = Aabb2d::new(wall_transform.translation.truncate(), wall.half_size)
                .closest_point(ball_pos);

            if closest_point == ball_pos {
                commands.entity(ball_entity).despawn();
                continue 'ball;
            }

            let diff = ball_pos - closest_point;
            if diff.length_squared() < ball.radius * ball.radius {
                let normal = diff.normalize();
                velocity.0 = velocity.reflect(normal);
                ball_transform.translation += (normal * (ball.radius - diff.length())).extend(0.);
            }
        }

        // Check for paddle collisions
        for (paddle, paddle_team, paddle_transform) in q_paddle.iter() {
            let paddle_aabb =
                Aabb2d::new(paddle_transform.translation.truncate(), paddle.half_size);
            let closest_point = paddle_aabb.closest_point(ball_pos);

            let diff = ball_pos - closest_point;
            if diff.length_squared() < ball.radius * ball.radius {
                let normal = diff.normalize();

                // Calculate reflection angle based on hit position
                let hit_position =
                    (ball_pos.x - paddle_transform.translation.x) / paddle.half_size.x;
                let angle = hit_position * std::f32::consts::PI / 3.0; // Max 60 degrees

                let speed = velocity.length();
                let dir = 1. - 2. * paddle_team.0 as f32; // Team 0: up, Team 1: down
                velocity.0 = Vec2::new(angle.sin(), dir * angle.cos()) * speed;
                ball_transform.translation += (normal * (ball.radius - diff.length())).extend(0.);
                continue 'ball;
            }
        }

        // Check for cell collisions
        for (cell_entity, cell, cell_team, cell_transform) in q_cell.iter() {
            if cell_team != ball_team {
                continue;
            }
            let closest_point = Aabb2d::new(cell_transform.translation.truncate(), cell.half_size)
                .closest_point(ball_pos);

            if closest_point == ball_pos {
                commands.entity(ball_entity).despawn();
                continue 'ball;
            }

            let diff = ball_pos - closest_point;
            if diff.length_squared() < ball.radius * ball.radius {
                let normal = diff.normalize();
                velocity.0 = velocity.reflect(normal);
                ball_transform.translation += (normal * (ball.radius - diff.length())).extend(0.);
                events.write(CellClicked { cell: cell_entity });
                continue 'ball;
            }
        }
    }
}
