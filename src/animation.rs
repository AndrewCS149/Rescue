use crate::components::{AnimationIndexRange, AnimationTimer, Direction, IsMoving, Player};
use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite)
            .add_system(change_animation::<Player>);
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
    )>,
) {
    for (mut timer, index_range, mut sprite, is_moving) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            // run animations if the player is actively moving
            if is_moving.0 {
                if index_range.1 == 0 {
                    sprite.index = 0;
                } else if !(index_range.0..index_range.1).contains(&sprite.index) {
                    sprite.index = index_range.0;
                } else {
                    sprite.index = sprite.index + 1;
                }
            // if player is not moving, set the sprite index to the first frame in the current range of animation indexes
            } else {
                sprite.index = index_range.0;
            }
        }
    }
}

fn change_animation<T: Component>(
    mut player_query: Query<(&mut AnimationIndexRange, &Direction), With<T>>,
) {
    for (mut index_range, direction) in player_query.iter_mut() {
        let new_index_range = match direction {
            Direction::Left => (1, 4),
            Direction::Right => (5, 8),
            Direction::Down => (0, 0),
            Direction::Up => (0, 0),
        };

        index_range.0 = new_index_range.0;
        index_range.1 = new_index_range.1;
    }
}
