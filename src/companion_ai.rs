use bevy::prelude::*;

use crate::ai::ai;
use crate::components::{Companion, Player, Speed};

const STAY_WITHIN: f32 = 100.0;

pub struct CompanionAIPlugin;

impl Plugin for CompanionAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tracking);
    }
}

// companion will track towards the player if the player is further than the 'STAY_WITHIN' const value
fn tracking(
    time: Res<Time>,
    mut companion: Query<(&mut Transform, &Speed), With<Companion>>,
    player: Query<(&Transform, Without<Companion>, With<Player>)>,
) {
    for (mut companion_transform, companion_speed) in companion.iter_mut() {
        let player_pos = player.single().0.translation;
        let companion_pos = companion_transform.translation;

        // if player and companion distance is greater than the STAY_WITHIN const value
        if (companion_pos.x - player_pos.x).abs() > STAY_WITHIN
            || (companion_pos.y - player_pos.y).abs() > STAY_WITHIN
        {
            let new_pos = ai::tracking(companion_pos, player_pos);
            companion_transform.translation += new_pos * companion_speed.0 * time.delta_seconds();
        }
    }
}
