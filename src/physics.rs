use crate::components::{Speed, Sprint};
use bevy::prelude::*;

pub fn movement<T: Component>(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed, &Sprint, With<T>)>,
) {
    for (mut transform, speed, sprint, _) in query.iter_mut() {
        let mut new_pos = Vec3::new(0.0, 0.0, 0.0);
        let mut tmp_sprint = 1.0;

        // left
        if keys.pressed(KeyCode::A) {
            new_pos.x = -1.0;
        }

        // right
        if keys.pressed(KeyCode::D) {
            new_pos.x = 1.0;
        }

        // up
        if keys.pressed(KeyCode::W) {
            new_pos.y = 1.0;
        }

        // down
        if keys.pressed(KeyCode::S) {
            new_pos.y = -1.0;
        }

        // sprint
        if keys.pressed(KeyCode::LShift) {
            tmp_sprint = sprint.0;
        }

        transform.translation += new_pos * speed.0 * tmp_sprint * time.delta_seconds();
    }
}
