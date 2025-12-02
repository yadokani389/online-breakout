use bevy::{
    color::palettes::css,
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use bevy_ggrs::prelude::*;

use super::{
    GameState,
    ball::{BALL_RADIUS, Ball, Velocity},
    components::{Count, Team},
    field::{CELL_SIZE, Cell, CellClicked, FIELD_WIDTH, toggle_cell},
    paddle::{PADDLE_HEIGHT, Paddle},
};

const ITEM_FALL_SPEED: f32 = 150.0;
const MULTI_BALL_COUNT: i32 = 2;
const MAX_BALL_SPEED: f32 = 60000000.0;
const MAX_BALL_COUNT: usize = 20;
const SPEED_UP_MULTIPLIER: f32 = 1.2;
const ENLARGE_PADDLE_MULTIPLIER: f32 = 1.5;
const ITEM_SIZE: f32 = 20.0;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            GgrsSchedule,
            (
                spawn_item,
                move_items,
                check_paddle_collision,
                apply_item_effect,
            )
                .chain()
                .before(toggle_cell)
                .run_if(in_state(GameState::InGame)),
        )
        .rollback_component_with_copy::<Item>()
        .add_message::<ItemCollected>();
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ItemType {
    EnlargePaddle,
    SpeedUp,
    MultiBall,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Item {
    item_type: ItemType,
}

#[derive(Message)]
struct ItemCollected {
    team: Team,
    item_type: ItemType,
}

pub fn spawn_item(
    mut commands: Commands,
    mut ev: MessageReader<CellClicked>,
    q_cell: Query<(&Transform, &Team), With<Cell>>,
    mut count: Local<Count>,
) {
    for ev in ev.read() {
        let Ok((cell_transform, cell_team)) = q_cell.get(ev.cell) else {
            continue;
        };
        if *cell_team != Team::ITEM {
            continue;
        }
        let item_type = match count.0 % 3 {
            0 => ItemType::EnlargePaddle,
            1 => ItemType::SpeedUp,
            _ => ItemType::MultiBall,
        };
        count.0 += 1;

        commands
            .spawn((
                Item { item_type },
                ev.team,
                Sprite::from_color(css::YELLOW, Vec2::splat(ITEM_SIZE)),
                Transform::from_translation(cell_transform.translation),
            ))
            .add_rollback();
    }
}

fn move_items(q_items: Query<(&Team, &mut Transform), With<Item>>, time: Res<Time>) {
    for (team, mut transform) in q_items {
        let direction = if team.0 == 0 { -1.0 } else { 1.0 };
        transform.translation.y += direction * ITEM_FALL_SPEED * time.delta_secs();
    }
}

fn check_paddle_collision(
    mut commands: Commands,
    q_items: Query<(Entity, &Item, &Team, &Transform)>,
    q_paddles: Query<(&Paddle, &Team, &Transform)>,
    mut ev_collision: MessageWriter<ItemCollected>,
) {
    for (item_entity, item, team, item_transform) in q_items {
        for (paddle, paddle_team, paddle_transform) in q_paddles {
            if *team != *paddle_team {
                continue;
            }

            let paddle_aabb2 =
                Aabb2d::new(paddle_transform.translation.truncate(), paddle.half_size);
            let item_aabb2 = Aabb2d::new(
                item_transform.translation.truncate(),
                Vec2::splat(ITEM_SIZE),
            );

            if paddle_aabb2.intersects(&item_aabb2) {
                commands.entity(item_entity).despawn();
                ev_collision.write(ItemCollected {
                    team: *paddle_team,
                    item_type: item.item_type,
                });
            }
        }
    }
}

fn apply_item_effect(
    mut commands: Commands,
    mut ev_collected: MessageReader<ItemCollected>,
    mut q_balls: Query<(&Ball, &Team, &Transform, &mut Velocity)>,
    mut q_paddles: Query<(&mut Paddle, &mut Mesh2d, &Team)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ev in ev_collected.read() {
        match ev.item_type {
            ItemType::MultiBall => {
                if MAX_BALL_COUNT <= q_balls.iter().count() {
                    continue;
                }
                for (ball, team, transform, velocity) in &q_balls {
                    if *team != ev.team {
                        continue;
                    }

                    for i in 0..MULTI_BALL_COUNT {
                        let angle = std::f32::consts::PI / 6.0 * (i as f32 - 0.5);
                        let new_velocity = Vec2::new(
                            velocity.x * angle.cos() - velocity.y * angle.sin(),
                            velocity.x * angle.sin() + velocity.y * angle.cos(),
                        );

                        commands
                            .spawn((
                                *ball,
                                *team,
                                Mesh2d(meshes.add(Mesh::from(Circle::new(BALL_RADIUS)))),
                                MeshMaterial2d(materials.add(Color::BLACK)),
                                Transform::from_translation(transform.translation),
                                Velocity(new_velocity),
                            ))
                            .add_rollback();
                    }
                }
            }
            ItemType::SpeedUp => {
                for (_, team, _, mut velocity) in &mut q_balls {
                    if *team == ev.team {
                        velocity.0 *= SPEED_UP_MULTIPLIER;
                        if MAX_BALL_SPEED.powf(2.) < velocity.length_squared() {
                            velocity.0 = velocity.0.normalize() * MAX_BALL_SPEED;
                        }
                    }
                }
            }
            ItemType::EnlargePaddle => {
                for (mut paddle, mut mesh2d, team) in &mut q_paddles {
                    if *team == ev.team {
                        paddle.half_size.x = (paddle.half_size.x * ENLARGE_PADDLE_MULTIPLIER)
                            .min(FIELD_WIDTH as f32 * CELL_SIZE * 2. / 6.);
                        *mesh2d = Mesh2d(
                            meshes.add(Rectangle::new(2. * paddle.half_size.x, PADDLE_HEIGHT)),
                        );
                    }
                }
            }
        }
    }
}
