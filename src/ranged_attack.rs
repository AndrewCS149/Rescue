use crate::components::{
    Animation, AnimationIndexRange, AnimationTimer, Damage, Direction, IsAttacking, IsMoving,
    IsSprinting, Player, Projectile, Speed,
};
use bevy::prelude::*;

const SPEED: f32 = 1000.0;
const DAMAGE: f32 = 25.0;

pub struct RangedAttackPlugin;

impl Plugin for RangedAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(draw_bowstring)
            .add_system(shoot_arrow::<Player>)
            .add_system(arrow_movement)
            .add_system(despawn_projectile);
    }
}

// draws the bowstring while the player is holding 'J'
fn draw_bowstring(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Direction, &mut IsAttacking, &mut Animation), With<Player>>,
) {
    for (direction, mut is_attacking, mut animation) in query.iter_mut() {
        if keys.pressed(KeyCode::J) {
            is_attacking.0 = true;

            *animation = match direction {
                Direction::Left => Animation::ShootLeft,
                Direction::Right => Animation::ShootRight,
                Direction::Up => Animation::ShootUp,
                Direction::Down => Animation::ShootDown,
            };
        } else {
            is_attacking.0 = false;

            *animation = match direction {
                Direction::Left => Animation::WalkLeft,
                Direction::Right => Animation::WalkRight,
                Direction::Up => Animation::WalkUp,
                Direction::Down => Animation::WalkDown,
            };
        }
    }
}

// shoots an arrow after the player has drawn the bowstring and released 'J'
fn shoot_arrow<T: Component>(
    mut commands: Commands,
    assets: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &Transform,
            &Direction,
            &IsSprinting,
            &mut IsAttacking,
            &TextureAtlasSprite,
            &AnimationIndexRange,
        ),
        With<T>,
    >,
) {
    for (transform, direction, is_sprinting, mut is_attacking, sprite, idx_range) in
        query.iter_mut()
    {
        if keys.just_released(KeyCode::J) && !is_sprinting.0 && sprite.index == idx_range.1 - 1 {
            is_attacking.0 = false;

            // based on which direction the arrow is moving, choose either the X or Y arrow image and flip it if needed
            let image = match direction {
                Direction::Left => ("arrowX.png", true),
                Direction::Right => ("arrowX.png", false),
                Direction::Up => ("arrowY.png", false),
                Direction::Down => ("arrowY.png", true),
            };

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
                .insert(Speed(SPEED))
                .insert(Damage(DAMAGE));
        }
    }
}

// controls the movement and directions of the projectiles
fn arrow_movement(
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
