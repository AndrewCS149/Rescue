use bevy::prelude::*;

use crate::components::{Companion, Player, Speed};

pub struct DogAIPlugin;

impl Plugin for DogAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(follow);
    }
}

fn follow(
    time: Res<Time>,
    mut companion: Query<(&mut Transform, &Speed), With<Companion>>,
    player: Query<(&Transform, Without<Companion>, With<Player>)>,
) {
    for (mut companion_transform, companion_speed) in companion.iter_mut() {
        let player_pos = player.single().0.translation;
        let companion_pos = companion_transform.translation;

        if (companion_pos.x - player_pos.x).abs() > 100.0
            || (companion_pos.y - player_pos.y).abs() > 100.0
        {
            let mut new_pos = Vec3::new(0.0, 0.0, 0.0);
            let buff = 0.25;

            let calc_new_pos = |e_pos: f32, p_pos: f32| match p_pos {
                num if e_pos > num + buff => -1.0,
                num if e_pos < num - buff => 1.0,
                _ => 0.0,
            };

            new_pos.x = calc_new_pos(companion_pos.x, player_pos.x);
            new_pos.y = calc_new_pos(companion_pos.y, player_pos.y);

            companion_transform.translation += new_pos * companion_speed.0 * time.delta_seconds();
        }
    }
}
