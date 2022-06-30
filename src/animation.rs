use crate::components::{Action, Animation, AnimationIndexRange, AnimationTimer, IsMoving, Player};
use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite)
            .add_system(change_animation::<Player>.before(animate_sprite));
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut AnimationIndexRange,
        &mut TextureAtlasSprite,
        &IsMoving,
        &Action,
    )>,
) {
    for (mut timer, idx_range, mut sprite, is_moving, action) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            // if is_attacking.0 || is_moving.0 {
            if *action == Action::RangedAttack || is_moving.0 {
                if !(idx_range.0..=idx_range.1).contains(&sprite.index) {
                    sprite.index = idx_range.0;
                } else if (idx_range.1 - 1..=idx_range.1).contains(&sprite.index)
                    && *action == Action::RangedAttack
                {
                } else {
                    sprite.index += 1;
                }
            } else {
                sprite.index = idx_range.0;
            }

            if sprite.index > idx_range.1 {
                sprite.index = idx_range.0;
            }
        }
    }
}

// changes the current animations start index and end index
fn change_animation<T: Component>(
    mut player_query: Query<(&mut AnimationIndexRange, &Animation), With<T>>,
) {
    for (mut idx_range, animation) in player_query.iter_mut() {
        *idx_range = match animation {
            Animation::WalkDown => AnimationIndexRange(0, 3),
            Animation::WalkUp => AnimationIndexRange(4, 7),
            Animation::WalkLeft => AnimationIndexRange(8, 11),
            Animation::WalkRight => AnimationIndexRange(12, 15),
            Animation::ShootRight => AnimationIndexRange(16, 19),
            Animation::ShootLeft => AnimationIndexRange(20, 23),
            Animation::ShootUp => AnimationIndexRange(24, 27),
            Animation::ShootDown => AnimationIndexRange(28, 31),
            Animation::MeleeLeft => AnimationIndexRange(32, 33),
            Animation::MeleeRight => AnimationIndexRange(34, 35),
        };
    }
}
