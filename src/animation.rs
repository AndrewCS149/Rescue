use crate::components::{
    Animation, AnimationIndexRange, AnimationTimer, Direction, IsAttacking, IsMoving, Player,
};
use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite)
            .add_system(change_animation::<Player>.before(animate_sprite));
    }
}

// taken from the Bevy Github 2d spritesheet example
// https://github.com/bevyengine/bevy/blob/main/examples/2d/sprite_sheet.rs
pub fn animate_sprite(
    time: Res<Time>,
    // texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut AnimationIndexRange,
        &mut TextureAtlasSprite,
        &IsMoving,
        &mut IsAttacking,
    )>,
) {
    for (mut timer, index_range, mut sprite, is_moving, mut is_attacking) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            // run attack animations if player is attacking
            if is_attacking.0 {
                if !(index_range.0..index_range.1).contains(&sprite.index) {
                    sprite.index = index_range.0 - 1;
                } else if sprite.index == index_range.1 - 1 {
                    is_attacking.0 = false;
                }

                sprite.index += 1;
            }
            // run move animations if the player is actively moving
            else if is_moving.0 {
                if !(index_range.0..index_range.1).contains(&sprite.index) {
                    sprite.index = index_range.0;
                } else {
                    sprite.index += 1;
                }
            // if player is not moving, set the sprite index to the first frame in the current range of animation indexes
            } else {
                sprite.index = index_range.0;
            }
        }
    }
}

// changes the current animations start index and end index
fn change_animation<T: Component>(
    mut player_query: Query<
        (
            &mut AnimationIndexRange,
            &Direction,
            &IsAttacking,
            &Animation,
        ),
        With<T>,
    >,
) {
    for (mut index_range, direction, is_attacking, animation) in player_query.iter_mut() {
        *index_range = match is_attacking.0 {
            true => match animation {
                Animation::ShootRight => AnimationIndexRange(16, 19),
                Animation::ShootLeft => AnimationIndexRange(20, 23),
                Animation::ShootUp => AnimationIndexRange(24, 27),
                Animation::ShootDown => AnimationIndexRange(28, 31),
                Animation::MeleeLeft => AnimationIndexRange(32, 33),
                Animation::MeleeRight => AnimationIndexRange(34, 35),
            },
            false => match direction {
                Direction::Down => AnimationIndexRange(0, 3),
                Direction::Up => AnimationIndexRange(4, 7),
                Direction::Left => AnimationIndexRange(8, 11),
                Direction::Right => AnimationIndexRange(12, 15),
            },
        };
    }
}
