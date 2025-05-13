use std::sync::Arc;

use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use iroh_gossip_signaller::IrohGossipSignallerBuilder;
use matchbox_socket::{WebRtcSocket, WebRtcSocketBuilder};

use crate::game::Config;

pub mod direct_message;
pub mod iroh_gossip_signaller;

pub struct OnlinePlugin;

impl Plugin for OnlinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_tokio_tasks::TokioTasksPlugin::default())
            .add_systems(Startup, start_matchbox_socket)
            .add_systems(Update, wait_for_players);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct IrohSocket(WebRtcSocket);

fn start_matchbox_socket(runtime: ResMut<TokioTasksRuntime>) {
    runtime.spawn_background_task(|mut ctx| async move {
        let signaller_builder = IrohGossipSignallerBuilder::new().await.unwrap();
        let builder = WebRtcSocketBuilder::new("")
            .signaller_builder(Arc::new(signaller_builder))
            .add_unreliable_channel();
        info!("Starting matchbox socket");
        let (socket, message_loop_fut) = builder.build();
        ctx.run_on_main_thread(move |ctx| {
            ctx.world.insert_resource(IrohSocket(socket));
        })
        .await;
        _ = message_loop_fut.await;
    });
}

fn wait_for_players(mut commands: Commands, socket: Option<ResMut<IrohSocket>>) {
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
}
