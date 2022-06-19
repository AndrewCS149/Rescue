use bevy::prelude::*;

mod animation;
mod components;
mod physics;
mod player;
mod projectile;
mod setup;

use animation::AnimationPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ProjectilePlugin)
        .run();
}
