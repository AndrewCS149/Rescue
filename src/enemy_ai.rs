use bevy::prelude::*;

use crate::ai::ai;
use crate::components::{BoundaryTrigger, Enemy, Player, Speed};

pub struct EnemyAIPlugin;

impl Plugin for EnemyAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tracking);
    }
}

// enemy will track towards player if the player is closer than the 'BoundaryTrigger.0' value
fn tracking(
    time: Res<Time>,
    mut enemy: Query<(&mut Transform, &Speed, &BoundaryTrigger, With<Enemy>)>,
    player: Query<(&Transform, With<Player>, Without<Enemy>)>,
) {
    for (mut enemy_transform, enemy_speed, boundary, _) in enemy.iter_mut() {
        let player_pos = player.single().0.translation;

        // if player & enemy distance is less than the 'BoundaryTrigger.0' value
        if enemy_transform
            .translation
            .abs_diff_eq(player_pos, boundary.0)
        {
            let new_pos = ai::tracking(enemy_transform.translation, player_pos);
            enemy_transform.translation += new_pos * enemy_speed.0 * time.delta_seconds();
        }
    }
}
