use bevy::ecs::resource::Resource;
use clap::Parser;

#[derive(Parser, Resource, Debug, Clone)]
pub struct Args {
    #[clap(short, long)]
    pub synctest: bool,
    #[clap(short, long, default_value = "")]
    pub iroh: String,
}
