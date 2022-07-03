use bevy::prelude::*;

use crate::components::{Collider, Enemy, EntitySize, Player};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collision::<Enemy>);
    }
}

fn collision<T: Component>(
    player_query: Query<(&Transform, &EntitySize), With<Player>>,
    mut enemy_query: Query<(
        &mut Transform,
        &EntitySize,
        With<T>,
        With<Collider>,
        Without<Player>,
    )>,
) {
    for (mut enemy_transform, enemy_size, _, _, _) in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation;
        let player_size = player_query.single().1;
        let player_pos = player_query.single().0.translation;

        // this gives the collision space more pixels to hit. If it were just one, the collision would almost never happen
        let half_player_x = player_size.x / 2.0;
        let half_player_y = player_size.y / 2.0;

        let half_enemy_x = enemy_size.x / 2.0;
        let half_enemy_y = enemy_size.y / 2.0;

        // RIGHT
        if enemy_pos.x - half_enemy_x <= player_pos.x + half_player_x
            && enemy_pos.x - half_enemy_x >= player_pos.x + half_player_x - half_player_x
            && enemy_pos.y - half_enemy_y <= player_pos.y + half_player_y
            && enemy_pos.y + half_enemy_y >= player_pos.y - half_player_y
        {
            enemy_transform.translation.x = player_pos.x + (player_size.x + enemy_size.x) / 2.0;
        }
        // LEFT
        else if enemy_pos.x + half_enemy_x >= player_pos.x - half_player_x
            && enemy_pos.x + half_enemy_x <= player_pos.x - half_player_x + half_player_x
            && enemy_pos.y - half_enemy_y <= player_pos.y + half_player_y
            && enemy_pos.y + half_enemy_y >= player_pos.y - half_player_y
        {
            enemy_transform.translation.x = player_pos.x - (player_size.x + enemy_size.x) / 2.0;
        }
        // TOP
        else if enemy_pos.x + half_enemy_x >= player_pos.x - half_player_x
            && enemy_pos.x - half_enemy_x <= player_pos.x + half_player_x
            && enemy_pos.y - half_enemy_y <= player_pos.y + half_player_y
            && enemy_pos.y - half_enemy_y >= player_pos.y + half_player_y - half_player_y
        {
            enemy_transform.translation.y = player_pos.y + (player_size.y + enemy_size.y) / 2.0;
        }
        // BOTTOM
        else if enemy_pos.x + half_enemy_x >= player_pos.x - half_player_x
            && enemy_pos.x - half_enemy_x <= player_pos.x + half_player_x
            && enemy_pos.y + half_enemy_y >= player_pos.y - half_player_y
            && enemy_pos.y + half_enemy_y <= player_pos.y - half_player_y + half_player_y
        {
            enemy_transform.translation.y = player_pos.y - (player_size.y + enemy_size.y) / 2.0;
        }
    }
}
