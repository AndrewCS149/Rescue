use bevy::prelude::*;

use crate::components::{Animation, Direction, IsAttacking, IsSprinting, Player};

pub struct MeleeAttackPlugin;

impl Plugin for MeleeAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attack::<Player>);
    }
}

fn attack<T: Component>(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Direction, &IsSprinting, &mut IsAttacking, &mut Animation), With<T>>,
) {
    for (direction, is_sprinting, mut is_attacking, mut animation) in query.iter_mut() {
        if keys.just_pressed(KeyCode::Space) && !is_sprinting.0 && !is_attacking.0 {
            is_attacking.0 = true;

            *animation = match direction {
                Direction::Left => Animation::MeleeLeft,
                Direction::Right => Animation::MeleeRight,
                Direction::Up => Animation::MeleeLeft,
                Direction::Down => Animation::MeleeRight,
            };
        }
    }
}
