use crate::components::{Direction, IsSprinting, Speed, Sprint};
use bevy::prelude::*;

pub fn movement<T: Component>(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &Speed,
            &Sprint,
            &mut Direction,
            &mut IsSprinting,
        ),
        With<T>,
    >,
) {
    for (mut transform, speed, sprint, mut direction, mut is_sprinting) in query.iter_mut() {
        let mut new_pos = Vec3::new(0.0, 0.0, 0.0);
        let mut tmp_sprint = 1.0;
        is_sprinting.0 = false;

        // left
        if keys.pressed(KeyCode::A) {
            new_pos.x = -1.0;
            *direction = Direction::Left;
        }

        // right
        if keys.pressed(KeyCode::D) {
            new_pos.x = 1.0;
            *direction = Direction::Right;
        }

        // up
        if keys.pressed(KeyCode::W) {
            new_pos.y = 1.0;
            *direction = Direction::Up;
        }

        // down
        if keys.pressed(KeyCode::S) {
            new_pos.y = -1.0;
            *direction = Direction::Down;
        }

        // sprint
        if keys.pressed(KeyCode::LShift) {
            tmp_sprint = sprint.0;
            is_sprinting.0 = true;
        }

        transform.translation += new_pos * speed.0 * tmp_sprint * time.delta_seconds();
    }
}
