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
            let mut tmp_sprint = 1.0;
            *action = Action::Walk;
            let mut params = (0.0, 0.0, *direction, *animation);

            // left
            if keys.pressed(KeyCode::A) {
                params = (-1.0, 0.0, Direction::Left, Animation::WalkLeft);
            // right
            } else if keys.pressed(KeyCode::D) {
                params = (1.0, 0.0, Direction::Right, Animation::WalkRight);
            // up
            } else if keys.pressed(KeyCode::W) {
                params = (0.0, 1.0, Direction::Up, Animation::WalkUp);
            // down
            } else if keys.pressed(KeyCode::S) {
                params = (0.0, -1.0, Direction::Down, Animation::WalkDown);
            } else {
                *action = Action::Idle;
            }

            // sprint
            if keys.pressed(KeyCode::LShift) && *action == Action::Walk {
                tmp_sprint = sprint.0;
                *action = Action::Sprint;
            }

            *direction = params.2;
            *animation = params.3;
            let new_pos = Vec3::new(params.0, params.1, 0.0);
            transform.translation += new_pos * speed.0 * tmp_sprint * time.delta_seconds();
        }
    }
}
