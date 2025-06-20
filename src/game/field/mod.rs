use bevy::prelude::*;
use bevy_ggrs::{LocalPlayers, prelude::*};

use super::{
    GameState,
    components::{Count, Team},
};

pub const FIELD_WIDTH: i32 = 10;
pub const FIELD_HEIGHT: i32 = 10;
pub const CELL_SIZE: f32 = 50.;
pub const CELL_THICKNESS: f32 = 5.;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CellClicked>()
            .add_systems(OnEnter(GameState::InGame), setup_field)
            .add_systems(
                GgrsSchedule,
                toggle_cell.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (rotate, update_cell_color).run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Clone, Copy, Component)]
pub struct Wall {
    pub half_size: Vec2,
}

#[derive(Component)]
pub struct Cell {
    pub half_size: Vec2,
}

#[derive(Event)]
pub struct CellClicked {
    pub cell: Entity,
    pub team: Team,
}

fn rotate(
    mut camera: Single<&mut Transform, With<Camera>>,
    local_players: Res<LocalPlayers>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    let Some(flag) = local_players.0.first().map(|x| *x == 1) else {
        warn!("No local player found, cannot rotate camera.");
        return;
    };
    if flag {
        camera.rotate_z(std::f32::consts::PI);
    }
    *done = true;
}

fn setup_field(mut commands: Commands) {
    // spawn cells
    for i in 0..2 {
        for x in -FIELD_WIDTH / 2..FIELD_WIDTH / 2 {
            for y in 0..FIELD_HEIGHT {
                let team = Team(i);
                commands
                    .spawn((
                        Cell {
                            half_size: Vec2::splat(CELL_SIZE / 2.),
                        },
                        team,
                        Sprite::from_color(
                            Color::hsl(team.hue(), 0.6, 0.7),
                            Vec2::splat(CELL_SIZE),
                        ),
                        Transform::from_xyz(
                            (x as f32 + 0.5) * CELL_SIZE,
                            ((2. * i as f32 - 1.) * (y as f32 + 0.5)) * CELL_SIZE,
                            5.,
                        ),
                        children![(
                            Sprite::from_color(
                                Color::hsl(team.hue(), 0.8, 0.7),
                                Vec2::splat(CELL_SIZE - CELL_THICKNESS)
                            ),
                            Transform::IDENTITY,
                        )],
                    ))
                    .add_rollback();
            }
        }
    }

    // spawn walls
    let wall_thickness = 1000.;
    let wall_width = FIELD_WIDTH as f32 * CELL_SIZE;
    let wall_height = FIELD_HEIGHT as f32 * CELL_SIZE * 2.;

    let half_size = Vec2::new(wall_width, wall_thickness) / 2.;

    commands.spawn((
        Wall { half_size },
        Team(1),
        Sprite::from_color(
            Color::srgb(0.3, 0.3, 0.3),
            Vec2::new(wall_width, wall_thickness),
        ),
        Transform::from_xyz(0., (wall_height + wall_thickness) / 2., 6.),
    ));
    commands.spawn((
        Wall { half_size },
        Team(0),
        Sprite::from_color(
            Color::srgb(0.3, 0.3, 0.3),
            Vec2::new(wall_width, wall_thickness),
        ),
        Transform::from_xyz(0., -(wall_height + wall_thickness) / 2., 6.),
    ));

    let half_size = Vec2::new(wall_thickness, wall_height + wall_thickness) / 2.;

    commands.spawn((
        Wall { half_size },
        Sprite::from_color(
            Color::srgb(0.3, 0.3, 0.3),
            Vec2::new(wall_thickness, wall_height + wall_thickness),
        ),
        Transform::from_xyz(-(wall_width + wall_thickness) / 2., 0., 6.),
    ));
    commands.spawn((
        Wall { half_size },
        Sprite::from_color(
            Color::srgb(0.3, 0.3, 0.3),
            Vec2::new(wall_thickness, wall_height + wall_thickness),
        ),
        Transform::from_xyz((wall_width + wall_thickness) / 2., 0., 6.),
    ));
}

pub fn toggle_cell(
    mut q_cell: Query<&mut Team, With<Cell>>,
    mut q_click: EventReader<CellClicked>,
    mut count: Local<Count>,
) {
    for event in q_click.read() {
        if let Ok(mut team) = q_cell.get_mut(event.cell) {
            if 10 <= count.0 {
                *team = Team::ITEM;
                count.0 = 0;
            } else {
                *team = event.team;
            }
            count.0 += 1;
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_cell_color(
    q_cell: Query<(&Children, &Team, &mut Sprite), (With<Cell>, Changed<Team>)>,
    mut q_child: Query<&mut Sprite, Without<Cell>>,
) {
    for (children, team, mut sprite) in q_cell {
        sprite.color = Color::hsl(team.hue(), 0.6, 0.7);
        for child in children {
            if let Ok(mut sprite) = q_child.get_mut(*child) {
                sprite.color = Color::hsl(team.hue(), 0.8, 0.7);
            }
        }
    }
}
