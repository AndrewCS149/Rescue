use crate::components::{Action, Animation, Direction, Player, Speed, Sprint};
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
            &mut Animation,
            &mut Action,
        ),
        With<T>,
    >,
) {
    for (mut transform, speed, sprint, mut direction, mut animation, mut action) in query.iter_mut()
    {
        if *action == Action::Idle || *action == Action::Walk || *action == Action::Sprint {
            let mut new_pos = Vec3::new(0.0, 0.0, 0.0);
            let mut tmp_sprint = 1.0;
            *action = Action::Walk;

            // left
            if keys.pressed(KeyCode::A) {
                new_pos.x = -1.0;
                *direction = Direction::Left;
                *animation = Animation::WalkLeft;
                *action = Action::Walk;
            }
            // right
            else if keys.pressed(KeyCode::D) {
                new_pos.x = 1.0;
                *direction = Direction::Right;
                *animation = Animation::WalkRight;
                *action = Action::Walk;
            }
            // up
            else if keys.pressed(KeyCode::W) {
                new_pos.y = 1.0;
                *direction = Direction::Up;
                *animation = Animation::WalkUp;
                *action = Action::Walk;
            }
            // down
            else if keys.pressed(KeyCode::S) {
                new_pos.y = -1.0;
                *direction = Direction::Down;
                *animation = Animation::WalkDown;
                *action = Action::Walk;
            } else {
                *action = Action::Idle;
            }

            // sprint
            if keys.pressed(KeyCode::LShift) && *action == Action::Walk {
                tmp_sprint = sprint.0;
                *action = Action::Sprint;
            }

            transform.translation += new_pos * speed.0 * tmp_sprint * time.delta_seconds();
        }
    }
}
