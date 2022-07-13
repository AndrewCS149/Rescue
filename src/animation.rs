use crate::components::{
    Action, Animation, AnimationIndexRange, AnimationTimer, Direction, Player,
};
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
        &mut Action,
        &Direction,
        &mut Animation,
    )>,
) {
    for (mut timer, idx_rng, mut sprite, mut action, direction, mut animation) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            match *action {
                Action::Walk | Action::Sprint => {
                    if !(idx_rng.0..=idx_rng.1).contains(&sprite.index) {
                        sprite.index = idx_rng.0;
                    } else {
                        sprite.index += 1;
                    }
                }
                Action::RangedAttack => {
                    if !(idx_rng.0..=idx_rng.1).contains(&sprite.index) {
                        sprite.index = idx_rng.0;
                    } else if !(idx_rng.1 - 1..=idx_rng.1).contains(&sprite.index) {
                        sprite.index += 1;
                    }
                }
                Action::MeleeAttack => {
                    if !(idx_rng.0..=idx_rng.1).contains(&sprite.index) {
                        sprite.index = idx_rng.0;
                    } else if sprite.index == idx_rng.1 {
                        *animation = match direction {
                            Direction::Left => Animation::WalkLeft,
                            Direction::Right => Animation::WalkRight,
                            Direction::Up => Animation::WalkUp,
                            Direction::Down => Animation::WalkDown,
                        };
                        *action = Action::Walk;
                    } else {
                        sprite.index += 1;
                    }
                }
                Action::Idle => {
                    sprite.index = idx_rng.0;
                }
                _ => {}
            };

            if sprite.index > idx_rng.1 {
                sprite.index = idx_rng.0;
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
            Animation::ShootRight => AnimationIndexRange(32, 35),
            Animation::ShootLeft => AnimationIndexRange(36, 39),
            Animation::ShootUp => AnimationIndexRange(40, 42),
            Animation::ShootDown => AnimationIndexRange(43, 45),
            Animation::MeleeLeft => AnimationIndexRange(48, 50),
            Animation::MeleeRight => AnimationIndexRange(51, 53),
        };
    }
}
