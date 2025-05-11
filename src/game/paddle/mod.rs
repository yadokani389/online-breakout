use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use super::field::Wall;

const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 10.0;
const PADDLE_Y_POSITION: f32 = -450.0;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PaddleSpeed(300.))
            .add_systems(Startup, setup_paddle)
            .add_systems(Update, move_paddle);
    }
}

#[derive(Resource)]
pub struct PaddleSpeed(pub f32);

#[derive(Component)]
pub struct Paddle {
    pub half_size: Vec2,
}

fn setup_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let half_size = Vec2::new(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0);
    commands.spawn((
        Paddle { half_size },
        Mesh2d(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
        Transform::from_xyz(0.0, PADDLE_Y_POSITION, 7.0),
    ));
}

fn move_paddle(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    paddle_speed: Res<PaddleSpeed>,
    mut query: Query<(&Paddle, &mut Transform)>,
    query_walls: Query<(&Wall, &Transform), Without<Paddle>>,
) {
    for (paddle, mut paddle_transform) in query.iter_mut() {
        let mut direction = 0.0;

        if keyboard.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }

        if direction == 0.0 {
            return;
        }

        let movement = direction * paddle_speed.0 * time.delta_secs();
        let mut new_x = paddle_transform.translation.x + movement;

        // Check wall collision
        for (wall, wall_transform) in query_walls.iter() {
            let paddle_aabb = Aabb2d::new(
                paddle_transform.translation.truncate().with_x(new_x),
                paddle.half_size,
            );
            let wall_aabb = Aabb2d::new(wall_transform.translation.truncate(), wall.half_size);

            if paddle_aabb.intersects(&wall_aabb) {
                new_x = if paddle_transform.translation.x < wall_transform.translation.x {
                    wall_aabb.min.x - paddle.half_size.x
                } else {
                    wall_aabb.max.x + paddle.half_size.x
                };
            }
        }

        paddle_transform.translation.x = new_x;
    }
}
