use crate::components::{Direction, Speed, Sprint};
use bevy::prelude::*;

pub fn movement<T: Component>(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed, &Sprint, &mut Direction, With<T>)>,
) {
    for (mut transform, speed, sprint, mut direction, _) in query.iter_mut() {
        let mut new_pos = Vec3::new(0.0, 0.0, 0.0);
        let mut tmp_sprint = 1.0;

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
        }

        transform.translation += new_pos * speed.0 * tmp_sprint * time.delta_seconds();
    }
}
