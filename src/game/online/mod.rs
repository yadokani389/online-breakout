use std::sync::Arc;

use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_wasm_tasks::Tasks;
use iroh_gossip_signaller::IrohGossipSignallerBuilder;
use matchbox_socket::{WebRtcSocket, WebRtcSocketBuilder};

use crate::{args::Args, game::Config};

use super::GameState;

pub mod direct_message;
pub mod iroh_gossip_signaller;
pub mod network_role;

pub struct OnlinePlugin;

impl Plugin for OnlinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_wasm_tasks::TasksPlugin::default())
            .add_systems(
                OnEnter(GameState::Matchmaking),
                start_matchbox_socket.run_if(p2p_mode),
            )
            .add_systems(
                Update,
                (
                    wait_for_players.run_if(p2p_mode),
                    start_synctest_session.run_if(synctest_mode),
                )
                    .run_if(in_state(GameState::Matchmaking)),
            );
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct IrohSocket(WebRtcSocket);

fn start_matchbox_socket(tasks: Tasks, args: Res<Args>) {
    let iroh_address = args.iroh.clone();
    tasks.spawn_auto(async move |x| {
        let signaller_builder = IrohGossipSignallerBuilder::new().await.unwrap();
        let builder = WebRtcSocketBuilder::new(iroh_address)
            .signaller_builder(Arc::new(signaller_builder))
            .add_unreliable_channel();
        info!("Starting matchbox socket");
        let (socket, message_loop_fut) = builder.build();
        x.submit_on_main_thread(move |ctx| {
            ctx.world.insert_resource(IrohSocket(socket));
        });
        _ = message_loop_fut.await;
    });
}

fn wait_for_players(
    mut commands: Commands,
    socket: Option<ResMut<IrohSocket>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(mut socket) = socket else {
        return; // socket not ready yet
    };

    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));

    next_state.set(GameState::InGame);
}

fn start_synctest_session(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    info!("Starting synctest session");
    let num_players = 2;

    let mut session_builder = ggrs::SessionBuilder::<Config>::new().with_num_players(num_players);

    for i in 0..num_players {
        session_builder = session_builder
            .add_player(ggrs::PlayerType::Local, i)
            .expect("failed to add player");
    }

    let ggrs_session = session_builder
        .start_synctest_session()
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::SyncTest(ggrs_session));

    next_state.set(GameState::InGame);
}

fn synctest_mode(args: Res<Args>) -> bool {
    args.synctest
}

fn p2p_mode(args: Res<Args>) -> bool {
    !args.synctest
}
