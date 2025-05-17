use bevy::prelude::*;
use clap::Parser;

mod args;
mod game;

fn main() {
    let args = get_args();
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // fill the entire browser window
                    fit_canvas_to_parent: true,
                    // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            game::GamePlugin,
        ))
        .insert_resource(args)
        .run();
}

fn get_args() -> args::Args {
    #[cfg(not(target_arch = "wasm32"))]
    {
        args::Args::parse()
    }

    #[cfg(target_arch = "wasm32")]
    {
        let mut args = args::Args::parse();
        // Get window.location object
        let window = web_sys::window().expect("no global `window` exists");
        let location = window.location();

        // Get the hash part of the URL (including the # symbol)
        if let Ok(hash) = location.hash() {
            args.iroh = hash.trim_start_matches('#').to_string();
        }

        args
    }
}
