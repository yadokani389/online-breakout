use bevy::prelude::*;
use clap::Parser;

mod args;
mod game;

fn main() {
    let args = args::Args::parse();
    App::new()
        .add_plugins((DefaultPlugins, game::GamePlugin))
        .insert_resource(args)
        .run();
}
