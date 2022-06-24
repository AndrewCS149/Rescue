use bevy::prelude::*;

mod animation;
mod components;
mod enemy;
mod map;
mod physics;
mod player;
mod projectile;
mod setup;

use animation::AnimationPlugin;
use enemy::EnemyPlugin;
use map::MapPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ProjectilePlugin)
        .run();
}
