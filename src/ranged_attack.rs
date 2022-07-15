use std::time::Duration;

use crate::{
    components::{
        Action, Animation, AnimationIndexRange, Arrow, Damage, Direction, Enemy, Health, Hurting,
        Player, Speed,
    },
    shared::health,
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
            .add_system(despawn_arrow)
            .add_system(damage);
    }
}

// draws the bowstring while the player is holding 'J'
fn draw_bowstring(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Direction, &mut Animation, &mut Action), With<Player>>,
) {
    for (direction, mut animation, mut action) in query.iter_mut() {
        // if user is holding fire (J) key, begin the draw bowstring/shooting animation
        if keys.pressed(KeyCode::J) {
            *action = Action::RangedAttack;
            *animation = match direction {
                Direction::Left => Animation::ShootLeft,
                Direction::Right => Animation::ShootRight,
                Direction::Up => Animation::ShootUp,
                Direction::Down => Animation::ShootDown,
            };
        // if user releases fire key before bow is fully drawn, reset to idle
        } else if keys.just_released(KeyCode::J) {
            *action = Action::Idle;
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
    audio: Res<Audio>,
    mut query: Query<
        (
            &Transform,
            &Direction,
            &mut TextureAtlasSprite,
            &AnimationIndexRange,
        ),
        With<T>,
    >,
) {
    for (transform, direction, mut sprite, idx_range) in query.iter_mut() {
        // if has released the fire (J) key while the bow is fully draw (sprite.idx == idx_rng.1 - 1)
        if keys.just_released(KeyCode::J) && sprite.index == idx_range.1 - 1 {
            // increase idx to play final frame
            sprite.index += 1;

            // based on which direction the player is currently facing, choose either the X or Y arrow image and flip it if needed
            let image = match direction {
                Direction::Left => ("arrowX.png", true),
                Direction::Right => ("arrowX.png", false),
                Direction::Up => ("arrowY.png", false),
                Direction::Down => ("arrowY.png", true),
            };

            // play fire arrow sound effect
            audio.play(assets.load("fire_arrow.ogg"));

            // create an arrow
            let arrow = SpriteBundle {
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
                .spawn_bundle(arrow)
                .insert(Arrow)
                .insert(*direction)
                .insert(Speed(SPEED))
                .insert(Damage(DAMAGE));
        }
    }
}

// controls the movement and direction of the arrows
fn arrow_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Direction, &Speed), With<Arrow>>,
) {
    for (mut transform, direction, speed) in query.iter_mut() {
        // move the projectile in the direction of the player's current direction
        let new_pos = match direction {
            Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Right => Vec3::new(1.0, 0.0, 0.0),
            Direction::Up => Vec3::new(0.0, 1.0, 0.0),
            Direction::Down => Vec3::new(0.0, -1.0, 0.0),
        };

        transform.translation += new_pos * speed.0 * time.delta_seconds();
    }
}

// despawn the arrow if it is outside of the window bounds
fn despawn_arrow(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    projectile: Query<(Entity, &Transform), With<Arrow>>,
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

// remove health from the enemy and destroys the enemy (death) when all health is depleted
fn damage(
    mut commands: Commands,
    arrow: Query<(Entity, &Transform, &Damage), With<Arrow>>,
    mut enemy: Query<
        (
            Entity,
            &Transform,
            &mut Sprite,
            &mut Hurting,
            &mut Health,
            Without<Arrow>,
        ),
        With<Enemy>,
    >,
) {
    if let Some((arrow, arrow_pos, damage)) = arrow.iter().next() {
        for (enemy, enemy_pos, mut sprite, mut hurting, mut health, _) in enemy.iter_mut() {
            if enemy_pos.translation.distance(arrow_pos.translation)
                < sprite.custom_size.unwrap().x / 2.0
            {
                // despawn projectile when contact with enemy is made
                commands.entity(arrow).despawn();

                health.current -= damage.0;

                // despawn healthbar and create a new updated healthbar
                commands.entity(enemy).despawn_descendants();
                let enemy_width = sprite.custom_size.unwrap().x;
                let updated_healthbar =
                    health::update_healthbar(enemy_width, health.current, health.total);

                // spawn new healthbar
                commands.entity(enemy).with_children(|parent| {
                    parent.spawn_bundle(updated_healthbar);
                });

                // turn enemy red for 'hurting' effect
                hurting.0 = Timer::new(Duration::from_millis(200), false);
                sprite.color = Color::RED;
            }
        }
    }
}
