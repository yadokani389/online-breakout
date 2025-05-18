use bevy::ecs::resource::Resource;

#[derive(Resource, Clone, Copy, Debug)]
pub enum NetworkRole {
    Host,
    Client,
}

impl NetworkRole {
    pub fn to_button_text(self) -> &'static str {
        match self {
            NetworkRole::Host => "Host a Game",
            NetworkRole::Client => "Join Game",
        }
    }
}
