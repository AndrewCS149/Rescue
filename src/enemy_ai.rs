use bevy::prelude::*;

use crate::components::{BoundaryTrigger, Enemy, Player, Speed};

pub struct EnemyAIPlugin;

impl Plugin for EnemyAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enemy_tracking);
    }
}

fn enemy_tracking(
    time: Res<Time>,
    mut enemy: Query<(&mut Transform, &Speed, &BoundaryTrigger, With<Enemy>)>,
    player: Query<(&Transform, With<Player>, Without<Enemy>)>,
) {
    for (mut enemy_transform, enemy_speed, boundary, _) in enemy.iter_mut() {
        let player_pos = player.single().0.translation;

        // only start tracking if within the set boundary trigger
        if enemy_transform
            .translation
            .abs_diff_eq(player_pos, boundary.0)
        {
            let enemy_pos = enemy_transform.translation;
            let mut new_pos = Vec3::new(0.0, 0.0, 0.0);
            let buff = 0.25;

            let calc_new_pos = |e_pos: f32, p_pos: f32| match p_pos {
                num if e_pos > num + buff => -1.0,
                num if e_pos < num - buff => 1.0,
                _ => 0.0,
            };

            new_pos.x = calc_new_pos(enemy_pos.x, player_pos.x);
            new_pos.y = calc_new_pos(enemy_pos.y, player_pos.y);

            enemy_transform.translation += new_pos * enemy_speed.0 * time.delta_seconds();
        }
    }
}
