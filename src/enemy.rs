use bevy::prelude::*;

use crate::components::{
    BoundaryTrigger, Collider, Damage, Enemy, EntitySize, Health, Projectile, Speed,
};

const HEALTH: f32 = 200.0;
const SPEED: f32 = 125.0;
const SIZE_X: f32 = 30.0;
const SIZE_Y: f32 = 30.0;
const BOUNDARY_TRIGGER: f32 = 250.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy).add_system(enemy_death);
    }
}

fn spawn_enemy(mut commands: Commands, assets: Res<AssetServer>) {
    let healthbar = SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(30.0, 3.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 20.0, 1.0),
        ..default()
    };

    let enemy = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE_X, SIZE_Y)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 100.0, 1.0),
        texture: assets.load("chicken.png"),
        ..default()
    };

    commands
        .spawn_bundle(enemy)
        .with_children(|parent| {
            parent.spawn_bundle(healthbar);
        })
        .insert(Enemy)
        .insert(EntitySize {
            x: SIZE_X,
            y: SIZE_Y,
        })
        .insert(Collider)
        .insert(Speed(SPEED))
        .insert(Health(HEALTH))
        .insert(BoundaryTrigger(BOUNDARY_TRIGGER));
}

fn enemy_death(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Transform, &Damage), With<Projectile>>,
    mut enemy_query: Query<(
        Entity,
        &Transform,
        &Sprite,
        &mut Health,
        With<Enemy>,
        Without<Projectile>,
    )>,
) {
    if let Some((projectile, projectile_pos, damage)) = projectile_query.iter().next() {
        for (enemy, enemy_pos, sprite, mut health, _, _) in enemy_query.iter_mut() {
            if enemy_pos.translation.distance(projectile_pos.translation)
                < sprite.custom_size.unwrap().x / 2.0
            {
                // despawn projectile when contact with enemy is made
                commands.entity(projectile).despawn();

                health.0 -= damage.0;

                // depsawn enemy if health is at or below 0.
                if health.0 <= 0.0 {
                    commands.entity(enemy).despawn_recursive();
                } else {
                    let enemy_width = sprite.custom_size.unwrap().x;

                    // compute new healthbar size, color and location
                    let remaining_health = enemy_width / (HEALTH / health.0);
                    let x_pos = -((remaining_health - enemy_width).abs() / 2.0);
                    let new_color = match health.0 {
                        h if h < HEALTH / 4.0 => Color::ORANGE,
                        h if h < HEALTH / 2.0 => Color::YELLOW,
                        _ => Color::GREEN,
                    };

                    // create a new healthbar with updated health color and location
                    let updated_healthbar = SpriteBundle {
                        sprite: Sprite {
                            color: new_color,
                            custom_size: Some(Vec2::new(remaining_health, 3.0)),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(x_pos, 20.0, 0.0),
                            ..default()
                        },
                        ..default()
                    };

                    // despawn the old health bar and spawn the new updated healthbar
                    commands.entity(enemy).despawn_descendants();
                    commands.entity(enemy).with_children(|parent| {
                        parent.spawn_bundle(updated_healthbar);
                    });
                }
            }
        }
    }
}
