use std::fmt::Debug;

use bevy::{color::palettes::css::GRAY, prelude::*};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::{
    args::Args,
    game::{GameState, online::network_role::NetworkRole},
};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

pub struct LobbyPlugin;

impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Lobby), setup_lobby)
            .add_systems(
                EguiPrimaryContextPass,
                show_textbox.run_if(in_state(GameState::Lobby)),
            )
            .add_systems(Update, button_system.run_if(in_state(GameState::Lobby)));
    }
}

fn setup_lobby(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(GameState::Lobby),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
        ))
        .with_children(|parent| {
            spawn_button(parent, NetworkRole::Host);
            spawn_button(parent, NetworkRole::Client);
        });
}

fn show_textbox(mut context: EguiContexts, mut args: ResMut<Args>) {
    egui::Area::new(egui::Id::new(0))
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(context.ctx_mut().unwrap(), |ui| {
            ui.label("Enter Room ID:");
            ui.add(
                egui::TextEdit::singleline(&mut args.iroh)
                    .hint_text("x".repeat(64))
                    .font(egui::FontId::proportional(30.))
                    .desired_width(400.),
            );
        });
}

#[allow(clippy::type_complexity)]
fn button_system(
    query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.set_all(GRAY);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.set_all(Color::WHITE);
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.set_all(Color::BLACK);
            }
        }
    }
}

fn spawn_button(parent: &mut ChildSpawnerCommands, role: NetworkRole) {
    parent
        .spawn((
            DespawnOnExit(GameState::Lobby),
            Button,
            Pickable::default(),
            Node {
                width: Val::Px(400.0),
                height: Val::Px(100.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new(role.to_button_text()),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )],
        ))
        .observe(on_click::<Pointer<Click>>(role));
}

fn on_click<E: Debug + Clone + Reflect + Event>(
    role: NetworkRole,
) -> impl Fn(On<E>, Commands, Res<Args>, ResMut<NextState<GameState>>) {
    move |_ev, mut commands, args, mut next_state| {
        if matches!(role, NetworkRole::Client) && 64 != args.iroh.len() {
            return;
        }
        commands.insert_resource(role);
        next_state.set(GameState::Matchmaking);
    }
}
