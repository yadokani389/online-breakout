use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_ggrs::prelude::*;

use super::{GameState, components::Team, field::Cell};

pub const GAME_DURATION_SECS: f32 = 120.; // 2 minutes

pub struct TimerPlugin;

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (start_game_timer, setup_timer_ui),
        )
        .add_systems(
            GgrsSchedule,
            check_victory_conditions
                .run_if(in_state(GameState::InGame))
                .run_if(resource_exists::<GameTimer>)
                .after(super::field::toggle_cell),
        )
        .add_systems(
            Update,
            update_timer_ui
                .run_if(in_state(GameState::InGame))
                .run_if(resource_exists::<GameTimer>),
        )
        .rollback_resource_with_clone::<GameTimer>();
    }
}

#[derive(Resource, Clone)]
pub struct GameTimer(pub Timer);

#[derive(Resource)]
pub struct GameResult {
    pub winner: Option<Team>,
    pub team0_blocks: usize,
    pub team1_blocks: usize,
}

fn start_game_timer(mut commands: Commands) {
    commands.insert_resource(GameTimer(Timer::from_seconds(
        GAME_DURATION_SECS,
        TimerMode::Once,
    )));
}

fn check_victory_conditions(
    mut commands: Commands,
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
    q_cells: Query<&Team, With<Cell>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        // Count blocks for each team
        let mut team0_blocks = 0;
        let mut team1_blocks = 0;

        for team in q_cells.iter() {
            match team.0 {
                0 => team0_blocks += 1,
                1 => team1_blocks += 1,
                _ => {} // Items don't count
            }
        }

        let winner = match team0_blocks.cmp(&team1_blocks) {
            Ordering::Greater => Some(Team(0)),
            Ordering::Less => Some(Team(1)),
            Ordering::Equal => None, // Draw
        };

        commands.insert_resource(GameResult {
            winner,
            team0_blocks,
            team1_blocks,
        });

        next_state.set(GameState::GameOver);
    }
}

#[derive(Component)]
struct TimerBarContainer;

#[derive(Component)]
struct TimerBar;

fn setup_timer_ui(mut commands: Commands) {
    // Simple timer bar at the bottom of the screen
    commands.spawn((
        TimerBarContainer,
        DespawnOnExit(GameState::InGame),
        Node {
            width: Val::Px(400.0),
            height: Val::Px(20.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-200.0)), // Center horizontally
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        BorderColor::all(Color::WHITE),
        ZIndex(1000),
        children![(
            TimerBar,
            DespawnOnExit(GameState::InGame),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
        )],
    ));
}

fn update_timer_ui(
    timer: Res<GameTimer>,
    timer_bar: Single<(&mut Node, &mut BackgroundColor), With<TimerBar>>,
) {
    let (mut node, mut background_color) = timer_bar.into_inner();
    let progress = timer.0.fraction();
    let remaining_progress = 1.0 - progress;

    // Update width to show remaining time (horizontal bar)
    node.width = Val::Percent(remaining_progress * 100.0);

    // Update color based on remaining time
    let color = if remaining_progress > 0.5 {
        Color::srgb(0.2, 0.8, 0.2) // Green
    } else if remaining_progress > 0.2 {
        Color::srgb(0.8, 0.8, 0.2) // Yellow
    } else {
        Color::srgb(0.8, 0.2, 0.2) // Red
    };

    background_color.0 = color;
}
