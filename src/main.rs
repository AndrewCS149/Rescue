use bevy::prelude::*;

mod components;
mod player;
mod setup;

use player::PlayerPlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
