use crate::components::{
    Action, Animation, Direction, IsMoving, IsSprinting, Player, Speed, Sprint,
};
use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement::<Player>);
    }
}

// controls the WASD movement
fn movement<T: Component>(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &Speed,
            &Sprint,
            &mut Direction,
            &mut IsSprinting,
            &mut IsMoving,
            &mut Animation,
            &Action,
        ),
        With<T>,
    >,
) {
    for (
        mut transform,
        speed,
        sprint,
        mut direction,
        mut is_sprinting,
        mut is_moving,
        mut animation,
        action,
    ) in query.iter_mut()
    {
        // if !is_attacking.0 {
        if *action == Action::Walk {
            let mut new_pos = Vec3::new(0.0, 0.0, 0.0);
            let mut tmp_sprint = 1.0;
            is_sprinting.0 = false;
            is_moving.0 = true;

            // left
            if keys.pressed(KeyCode::A) {
                new_pos.x = -1.0;
                *direction = Direction::Left;
                *animation = Animation::WalkLeft
            }
            // right
            else if keys.pressed(KeyCode::D) {
                new_pos.x = 1.0;
                *direction = Direction::Right;
                *animation = Animation::WalkRight
            }
            // up
            else if keys.pressed(KeyCode::W) {
                new_pos.y = 1.0;
                *direction = Direction::Up;
                *animation = Animation::WalkUp
            }
            // down
            else if keys.pressed(KeyCode::S) {
                new_pos.y = -1.0;
                *direction = Direction::Down;
                *animation = Animation::WalkDown
            } else {
                is_moving.0 = false;
            }

            // sprint
            if keys.pressed(KeyCode::LShift) {
                tmp_sprint = sprint.0;
                is_sprinting.0 = true;
            }

            transform.translation += new_pos * speed.0 * tmp_sprint * time.delta_seconds();
        }
    }
}
