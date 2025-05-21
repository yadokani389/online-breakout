use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    platform::collections::HashMap,
    prelude::*,
};
use bevy_ggrs::{LocalInputs, LocalPlayers, PlayerInputs, prelude::*};

use super::{Config, components::Team, online::network_role::NetworkRole};
use super::{GameState, field::Wall};

const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 10.0;
const PADDLE_Y_POSITION: f32 = 450.0;

const INPUT_LEFT: u8 = 1 << 0;
const INPUT_RIGHT: u8 = 1 << 1;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PaddleSpeed(300.))
            .add_systems(OnEnter(GameState::InGame), setup_paddle)
            .add_systems(ReadInputs, read_local_inputs)
            .add_systems(GgrsSchedule, move_paddles);
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
    commands
        .spawn((
            Paddle { half_size },
            Team(0),
            Mesh2d(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.0, -PADDLE_Y_POSITION, 7.0),
        ))
        .add_rollback();
    commands
        .spawn((
            Paddle { half_size },
            Team(1),
            Mesh2d(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_xyz(0.0, PADDLE_Y_POSITION, 7.0),
        ))
        .add_rollback();
}

fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    local_players: Res<LocalPlayers>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
    role: Res<NetworkRole>,
) {
    let mut local_inputs = HashMap::new();
    let (camera, camera_transform) = *q_camera;

    for handle in &local_players.0 {
        let mut input = 0;
        if keys.pressed(KeyCode::ArrowLeft) {
            input |= INPUT_LEFT;
        }
        if keys.pressed(KeyCode::ArrowRight) {
            input |= INPUT_RIGHT;
        }

        for finger in touches.iter() {
            let Ok(pos) = camera.viewport_to_world_2d(camera_transform, finger.position()) else {
                continue;
            };
            if pos.x.is_sign_negative() {
                input |= INPUT_LEFT;
            } else {
                input |= INPUT_RIGHT;
            }
        }

        // Reverse input if the role is Host
        if matches!(*role, NetworkRole::Host) {
            input = ((input & INPUT_LEFT != 0) as u8 * INPUT_RIGHT)
                | ((input & INPUT_RIGHT != 0) as u8 * INPUT_LEFT);
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<Config>(local_inputs));
}

pub fn move_paddles(
    time: Res<Time>,
    paddle_speed: Res<PaddleSpeed>,
    inputs: Res<PlayerInputs<Config>>,
    query: Query<(&Paddle, &Team, &mut Transform)>,
    query_walls: Query<(&Wall, &Transform), Without<Paddle>>,
) {
    // HACK: `for (paddle, team, mut paddle_transform) in query` does not work for team 1
    query
        .into_iter()
        .for_each(|(paddle, team, mut paddle_transform)| {
            let (input, _) = inputs[team.0];
            let mut direction = 0.0;

            if input & INPUT_LEFT != 0 {
                direction -= 1.0;
            }
            if input & INPUT_RIGHT != 0 {
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
        });
}
