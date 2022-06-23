use bevy::prelude::*;

use crate::components::{Damage, Enemy, Health, Projectile};

const HEALTH: f32 = 200.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy).add_system(enemy_death);
        //     .add_system(physics::movement::<Player>);
    }
}

fn spawn_enemy(mut commands: Commands) {
    let healthbar = SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(30.0, 3.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 20.0, 0.0),
            ..default()
        },
        ..default()
    };

    let enemy = SpriteBundle {
        sprite: Sprite {
            color: Color::TOMATO,
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(200.0, 200.0, 0.0),
            ..default()
        },
        ..default()
    };

    commands
        .spawn_bundle(enemy)
        .with_children(|parent| {
            parent.spawn_bundle(healthbar);
        })
        .insert(Enemy)
        .insert(Health(HEALTH));
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
