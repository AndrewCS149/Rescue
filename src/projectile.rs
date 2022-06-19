use crate::components::{Direction, Player, Projectile, Speed};
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_projectile::<Player>)
            .add_system(projectile_movement);
    }
}

fn spawn_projectile<T: Component>(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    query: Query<(&Transform, &Direction), With<T>>,
) {
    // only if the player has pressed the fire (space) button
    if keys.just_pressed(KeyCode::Space) {
        let pos = query.single().0.translation;
        let direction = query.single().1;

        let sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 0.0), // use players translation
            ..default()
        };

        commands
            .spawn_bundle(sprite)
            .insert(Projectile)
            .insert(*direction)
            .insert(Speed(1000.0));
    }
}

fn projectile_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Direction, &Speed), With<Projectile>>,
) {
    for (mut transform, direction, speed) in query.iter_mut() {
        let new_pos = match direction {
            Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Right => Vec3::new(1.0, 0.0, 0.0),
            Direction::Up => Vec3::new(0.0, 1.0, 0.0),
            Direction::Down => Vec3::new(0.0, -1.0, 0.0),
        };

        transform.translation += new_pos * speed.0 * time.delta_seconds();
    }
}
