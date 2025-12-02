use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

use crate::game::{GameState, components::Team, timer::GameResult};

pub struct ResultPlugin;

impl Plugin for ResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_result_screen);
    }
}

fn setup_result_screen(
    mut commands: Commands,
    game_result: Option<Res<GameResult>>,
    local_players: Res<LocalPlayers>,
) {
    let Some(result) = game_result else {
        return;
    };

    // Get local player's team
    let local_team = local_players.0.first().copied().unwrap_or(0);

    // Victory message
    let (winner_text, winner_color, score_text) = match result.winner {
        Some(Team(0)) => {
            let personal_message = if local_team == 0 {
                "You Win!"
            } else {
                "You Lose!"
            };
            (
                personal_message,
                if local_team == 0 {
                    Color::hsl(Team(0).hue(), 0.8, 0.7)
                } else {
                    Color::srgb(0.8, 0.3, 0.3)
                },
                format!(
                    "Player 1: {} blocks vs Player 2: {} blocks",
                    result.team0_blocks, result.team1_blocks
                ),
            )
        }
        Some(Team(1)) => {
            let personal_message = if local_team == 1 {
                "You Win!"
            } else {
                "You Lose!"
            };
            (
                personal_message,
                if local_team == 1 {
                    Color::hsl(Team(1).hue(), 0.8, 0.7)
                } else {
                    Color::srgb(0.8, 0.3, 0.3)
                },
                format!(
                    "Player 2: {} blocks vs Player 1: {} blocks",
                    result.team1_blocks, result.team0_blocks
                ),
            )
        }
        None => (
            "It's a Draw!",
            Color::srgb(0.7, 0.7, 0.7),
            format!("Both players: {} blocks", result.team0_blocks),
        ),
        Some(_) => ("Game Over", Color::WHITE, "".to_string()),
    };

    // Main UI container
    commands.spawn((
        DespawnOnExit(GameState::GameOver),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        children![
            (
                Text::new("GAME OVER"),
                TextFont::from_font_size(48.0),
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ),
            (
                // Winner announcement
                Text::new(winner_text),
                TextFont::from_font_size(36.0),
                TextColor(winner_color),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ),
            (
                // Score display
                Text::new(score_text),
                TextFont::from_font_size(24.0),
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ),
        ],
    ));
}
