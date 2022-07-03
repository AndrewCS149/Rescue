use bevy::prelude::*;

use crate::ai::ai;
use crate::components::{BoundaryTrigger, Companion, Player, Speed};

pub struct CompanionAIPlugin;

impl Plugin for CompanionAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tracking);
    }
}

// companion will track towards the player if the player is further than the BoundaryTrigger.0 value
fn tracking(
    time: Res<Time>,
    mut companion: Query<(&mut Transform, &Speed, &BoundaryTrigger), With<Companion>>,
    player: Query<(&Transform, Without<Companion>, With<Player>)>,
) {
    for (mut companion_transform, companion_speed, boundary) in companion.iter_mut() {
        let player_pos = player.single().0.translation;
        let companion_pos = companion_transform.translation;

        // if player and companion distance is greater than the BoundaryTrigger.0 value
        if (companion_pos.x - player_pos.x).abs() > boundary.0
            || (companion_pos.y - player_pos.y).abs() > boundary.0
        {
            let new_pos = ai::tracking(companion_pos, player_pos);
            companion_transform.translation += new_pos * companion_speed.0 * time.delta_seconds();
        }
    }
}
