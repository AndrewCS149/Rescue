use crate::components::{
    Animation, Damage, Direction, IsAttacking, IsSprinting, Player, Projectile, Speed,
};
use bevy::prelude::*;

pub struct RangedAttackPlugin;

impl Plugin for RangedAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_projectile::<Player>)
            .add_system(projectile_movement)
            .add_system(despawn_projectile);
    }
}

// spawns a projectile (arrow) every time the player pressed the fire (space) key
fn spawn_projectile<T: Component>(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    assets: Res<AssetServer>,
    mut query: Query<
        (
            &Transform,
            &Direction,
            &IsSprinting,
            &mut IsAttacking,
            &mut Animation,
        ),
        With<T>,
    >,
) {
    for (transform, direction, is_sprinting, mut is_attacking, mut animation) in query.iter_mut() {
        // if the player has pressed the fire (space) button, is not sprinting and is not already attacking
        if keys.just_pressed(KeyCode::J) && !is_sprinting.0 && !is_attacking.0 {
            is_attacking.0 = true;

            // based on which direction the arrow is moving, choose either the X or Y arrow image and flip it if needed
            // change appropriate animation enum
            let (image, anim) = match direction {
                Direction::Left => (("arrowX.png", true), Animation::ShootLeft),
                Direction::Right => (("arrowX.png", false), Animation::ShootRight),
                Direction::Up => (("arrowY.png", false), Animation::ShootUp),
                Direction::Down => (("arrowY.png", true), Animation::ShootDown),
            };

            *animation = anim;

            let sprite = SpriteBundle {
                sprite: Sprite {
                    flip_x: image.1,
                    flip_y: image.1,
                    ..default()
                },
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    1.0,
                ),
                texture: assets.load(image.0),
                ..default()
            };

            commands
                .spawn_bundle(sprite)
                .insert(Projectile)
                .insert(*direction)
                .insert(Speed(1000.0))
                .insert(Damage(25.0));
        }
    }
}

// controls the movement and directions of the projectiles
fn projectile_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Direction, &Speed), With<Projectile>>,
) {
    for (mut transform, direction, speed) in query.iter_mut() {
        // move the projectile in the direction of the host's current direction
        let new_pos = match direction {
            Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Right => Vec3::new(1.0, 0.0, 0.0),
            Direction::Up => Vec3::new(0.0, 1.0, 0.0),
            Direction::Down => Vec3::new(0.0, -1.0, 0.0),
        };

        transform.translation += new_pos * speed.0 * time.delta_seconds();
    }
}

// despawn the projectile if it is outside of the window bounds
fn despawn_projectile(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    projectile: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (projectile, transform) in projectile.iter() {
        let window = windows.get_primary_mut().unwrap();
        if transform.translation.x > window.width() / 2.0
            || transform.translation.x < -(window.width() / 2.0)
            || transform.translation.y > window.height() / 2.0
            || transform.translation.y < -(window.height() / 2.0)
        {
            commands.entity(projectile).despawn();
        }
    }
}
