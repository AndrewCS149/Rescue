use bevy::prelude::*;

use crate::components::Player;

pub struct DogAIPlugin;

impl Plugin for DogAIPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(enemy_tracking);
    }
}

fn follow(player: Query<&Transform, With<Player>>) {}
