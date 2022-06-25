use bevy::prelude::*;

pub struct EnemyAIPlugin;

impl Plugin for EnemyAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy).add_system(enemy_death);
    }
}
