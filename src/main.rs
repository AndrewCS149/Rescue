use bevy::prelude::*;

mod animation;
mod collision;
mod components;
mod dog;
mod enemy;
mod enemy_ai;
mod map;
mod melee_attack;
mod physics;
mod player;
mod ranged_attack;
mod setup;

use animation::AnimationPlugin;
use collision::CollisionPlugin;
use dog::DogPlugin;
use enemy::EnemyPlugin;
use enemy_ai::EnemyAIPlugin;
use map::MapPlugin;
use melee_attack::MeleeAttackPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use ranged_attack::RangedAttackPlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(EnemyAIPlugin)
        .add_plugin(DogPlugin)
        .add_plugin(CollisionPlugin)
        .add_plugin(MeleeAttackPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RangedAttackPlugin)
        .run();
}
