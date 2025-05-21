use bevy::prelude::*;
use bevy_ggrs::prelude::*;

use crate::game::ball::check_collision;

use super::{GameState, components::Team};

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CellClicked>()
            .add_systems(OnEnter(GameState::InGame), setup_field)
            .add_systems(
                GgrsSchedule,
                toggle_cell
                    .run_if(in_state(GameState::InGame))
                    .after(check_collision),
            )
            .add_systems(
                Update,
                update_cell_color.run_if(in_state(GameState::InGame)),
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
}

fn setup_field(mut commands: Commands) {
    let cell_size = 50.;
    let cell_thickness = 5.;
    let field_width = 10;
    let field_height = 10;

    // spawn cells
    for i in 0..2 {
        for x in -field_width / 2..field_width / 2 {
            for y in 0..field_height {
                commands
                    .spawn((
                        Cell {
                            half_size: Vec2::splat(cell_size / 2.),
                        },
                        Team(i),
                        Sprite::from_color(
                            Color::hsl(180. * i as f32, 0.6, 0.7),
                            Vec2::splat(cell_size),
                        ),
                        Transform::from_xyz(
                            (x as f32 + 0.5) * cell_size,
                            ((1. - 2. * i as f32) * (y as f32 + 0.5)) * cell_size,
                            5.,
                        ),
                        children![(
                            Sprite::from_color(
                                Color::hsl(180. * i as f32, 0.8, 0.7),
                                Vec2::splat(cell_size - cell_thickness)
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
    let wall_width = field_width as f32 * cell_size;
    let wall_height = field_height as f32 * cell_size * 2.;

    let half_size = Vec2::new(wall_width, wall_thickness) / 2.;

    commands.spawn((
        Wall { half_size },
        Sprite::from_color(
            Color::srgb(0.3, 0.3, 0.3),
            Vec2::new(wall_width, wall_thickness),
        ),
        Transform::from_xyz(0., (wall_height + wall_thickness) / 2., 6.),
    ));
    commands.spawn((
        Wall { half_size },
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

fn toggle_cell(mut q_cell: Query<&mut Team, With<Cell>>, mut q_click: EventReader<CellClicked>) {
    for event in q_click.read() {
        if let Ok(mut team) = q_cell.get_mut(event.cell) {
            team.0 = 1 - **team;
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_cell_color(
    q_cell: Query<(&Children, &Team, &mut Sprite), (With<Cell>, Changed<Team>)>,
    mut q_child: Query<&mut Sprite, Without<Cell>>,
) {
    for (children, team, mut sprite) in q_cell {
        sprite.color = Color::hsl(180. * team.0 as f32, 0.6, 0.7);
        for child in children {
            if let Ok(mut sprite) = q_child.get_mut(*child) {
                sprite.color = Color::hsl(180. * team.0 as f32, 0.8, 0.7);
            }
        }
    }
}
