use bevy::prelude::*;

use crate::{
    components::{Action, Animation, Direction, Enemy, Health, IsMeleeRange, Player},
    shared::health,
};

pub struct MeleeAttackPlugin;

impl Plugin for MeleeAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attack::<Player>).add_system(receive_damage);
    }
}

// remove health from the enemy and destroys the enemy (death) when all health is depleted
fn receive_damage(
    mut player: Query<&Transform, With<Player>>,
    mut enemy: Query<(&Transform, &mut Sprite, &mut IsMeleeRange, Without<Player>), With<Enemy>>,
) {
    for player_transform in player.iter_mut() {
        for (enemy_transform, enemy_sprite, mut is_melee_range, _) in enemy.iter_mut() {
            if player_transform
                .translation
                .distance(enemy_transform.translation)
                < 40.0
            {
                is_melee_range.0 = true;
            } else {
                is_melee_range.0 = false;
            }
        }
    }
}

fn attack<T: Component>(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut enemy_query: Query<(Entity, &mut Health, &IsMeleeRange, &Sprite), With<Enemy>>,
    mut player_query: Query<(&Direction, &mut Action, &mut Animation), With<T>>,
) {
    // for (enemy, mut enemy_health, is_melee_range, enemy_sprite) in enemy_query.iter_mut() {
    for (direction, mut action, mut animation) in player_query.iter_mut() {
        if keys.just_pressed(KeyCode::Space) && *action != Action::MeleeAttack {
            *action = Action::MeleeAttack;

            *animation = match direction {
                Direction::Left => Animation::MeleeLeft,
                Direction::Right => Animation::MeleeRight,
                Direction::Up => Animation::MeleeUp,
                Direction::Down => Animation::MeleeDown,
            };

            // println!("{}", is_melee_range.0);
            if let Some((enemy, mut enemy_health, is_melee_range, enemy_sprite)) =
                enemy_query.iter_mut().next()
            {
                if is_melee_range.0 {
                    enemy_health.current -= 25.0;

                    // despawn healthbar and create a new updated healthbar
                    commands.entity(enemy).despawn_descendants();
                    let enemy_width = enemy_sprite.custom_size.unwrap().x;
                    let updated_healthbar = health::update_healthbar(
                        enemy_width,
                        enemy_health.current,
                        enemy_health.total,
                    );

                    // spawn new healthbar
                    commands.entity(enemy).with_children(|parent| {
                        parent.spawn_bundle(updated_healthbar);
                    });
                }
            }
        }
    }
}
