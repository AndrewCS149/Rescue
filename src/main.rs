use bevy::prelude::*;

mod components;
mod physics;
mod player;
mod projectile;
mod setup;

use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ProjectilePlugin)
        .run();
}
