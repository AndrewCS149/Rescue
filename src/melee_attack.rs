use bevy::prelude::*;

use crate::components::{Action, Animation, Direction, Player};

pub struct MeleeAttackPlugin;

impl Plugin for MeleeAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attack::<Player>);
    }
}

fn attack<T: Component>(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&Direction, &mut Action, &mut Animation), With<T>>,
) {
    for (direction, mut action, mut animation) in player_query.iter_mut() {
        if keys.just_pressed(KeyCode::Space) && *action != Action::MeleeAttack {
            *action = Action::MeleeAttack;

            *animation = match direction {
                Direction::Left => Animation::MeleeLeft,
                Direction::Right => Animation::MeleeRight,
                Direction::Up => Animation::MeleeUp,
                Direction::Down => Animation::MeleeDown,
            };
        }
    }
}
