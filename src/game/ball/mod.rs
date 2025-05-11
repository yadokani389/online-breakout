use crate::game::field::Cell;
use bevy::{math::bounding::Aabb2d, prelude::*};

use super::field::{CellClicked, Wall};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ball)
            .add_systems(Update, (apply_velocity, check_collision).chain());
    }
}

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub team: u8,
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

fn setup_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 10.0;
    commands.spawn((
        Ball { radius, team: 0 },
        Mesh2d(meshes.add(Mesh::from(Circle::new(radius)))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(0., -100., 10.),
        Velocity(Vec2::new(100., 200.)),
    ));
    commands.spawn((
        Ball { radius, team: 1 },
        Mesh2d(meshes.add(Mesh::from(Circle::new(radius)))),
        MeshMaterial2d(materials.add(Color::srgb(0., 0., 0.))),
        Transform::from_xyz(0., 100., 10.),
        Velocity(Vec2::new(200., 200.)),
    ));
}

fn apply_velocity(time: Res<Time>, mut q_ball: Query<(&Velocity, &mut Transform), With<Ball>>) {
    for (velocity, mut transform) in q_ball.iter_mut() {
        transform.translation += velocity.extend(0.) * time.delta_secs();
    }
}

fn check_collision(
    mut commands: Commands,
    mut q_ball: Query<(Entity, &Ball, &mut Transform, &mut Velocity)>,
    q_cell: Query<(Entity, &Cell, &Transform), Without<Ball>>,
    q_wall: Query<(&Wall, &Transform), Without<Ball>>,
    mut events: EventWriter<CellClicked>,
) {
    'ball: for (ball_entity, ball, mut ball_transform, mut velocity) in q_ball.iter_mut() {
        let ball_pos = ball_transform.translation.truncate();

        // Check for cell collisions
        for (cell_entity, cell, cell_transform) in q_cell.iter() {
            if cell.team != ball.team {
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
                velocity.0 = velocity.0.reflect(normal);
                ball_transform.translation += (normal * (ball.radius - diff.length())).extend(0.);
                events.write(CellClicked { cell: cell_entity });
                continue 'ball;
            }
        }

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
                velocity.0 = velocity.0.reflect(normal);
                ball_transform.translation += (normal * (ball.radius - diff.length())).extend(0.);
            }
        }
    }
}
