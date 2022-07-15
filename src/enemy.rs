use std::time::Duration;

use bevy::prelude::*;

use crate::components::{
    Arrow, BoundaryTrigger, Collider, Collision, Damage, Enemy, EntitySize, Health, Hurting, Speed,
};

const HEALTH: f32 = 200.0;
const SPEED: f32 = 125.0;
const SIZE_X: f32 = 30.0;
const SIZE_Y: f32 = 30.0;
const BOUNDARY_TRIGGER: f32 = 250.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn)
            .add_system(receive_damage)
            .add_system(death)
            .add_system(receive_damage_effect);
    }
}

// spawns an enemy with a healthbar
fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    let enemy = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE_X, SIZE_Y)),
            ..default()
        },
        transform: Transform::from_xyz(300.0, 100.0, 1.0),
        texture: assets.load("chicken.png"),
        ..default()
    };

    let healthbar = SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            custom_size: Some(Vec2::new(enemy.sprite.custom_size.unwrap().x, 3.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 20.0, 1.0),
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
        .insert(Hurting(Timer::new(Duration::from_secs(0), false)))
        .insert(Collision::None)
        .insert(Speed(SPEED))
        .insert(Health(HEALTH))
        .insert(BoundaryTrigger(BOUNDARY_TRIGGER));
}

// despawns the enemy if health is at or below zero
fn death(mut commands: Commands, enemy: Query<(Entity, &Health), With<Enemy>>) {
    for (enemy, health) in enemy.iter() {
        if health.0 <= 0.0 {
            commands.entity(enemy).despawn_recursive();
        }
    }
}

// remove health from the enemy and destroys the enemy (death) when all health is depleted
fn receive_damage(
    mut commands: Commands,
    arrow: Query<(Entity, &Transform, &Damage), With<Arrow>>,
    mut enemy: Query<
        (
            Entity,
            &Transform,
            &mut Sprite,
            &mut Health,
            &mut Hurting,
            Without<Arrow>,
        ),
        With<Enemy>,
    >,
) {
    if let Some((arrow, arrow_pos, damage)) = arrow.iter().next() {
        for (enemy, enemy_pos, mut sprite, mut health, mut hurting, _) in enemy.iter_mut() {
            if enemy_pos.translation.distance(arrow_pos.translation)
                < sprite.custom_size.unwrap().x / 2.0
            {
                // despawn projectile when contact with enemy is made
                commands.entity(arrow).despawn();

                health.0 -= damage.0;

                // despawn healthbar and create a new updated healthbar
                commands.entity(enemy).despawn_descendants();
                let enemy_width = sprite.custom_size.unwrap().x;
                let updated_healthbar = update_healthbar(enemy_width, health.0);

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

fn receive_damage_effect(
    time: Res<Time>,
    mut enemy: Query<(&mut Sprite, &mut Hurting), With<Enemy>>,
) {
    for (mut sprite, mut hurting) in enemy.iter_mut() {
        hurting.0.tick(time.delta());

        if hurting.0.just_finished() {
            sprite.color = Color::default();
        }
    }
}

// calculates new health bar size
fn update_healthbar(enemy_width: f32, health: f32) -> SpriteBundle {
    // compute new healthbar size, color and location
    let remaining_health = enemy_width / (HEALTH / health);
    let x_pos = -((remaining_health - enemy_width).abs() / 2.0);
    let new_color = match health {
        h if h < HEALTH / 4.0 => Color::ORANGE,
        h if h < HEALTH / 2.0 => Color::YELLOW,
        _ => Color::GREEN,
    };

    // create a new healthbar with updated health color and location
    SpriteBundle {
        sprite: Sprite {
            color: new_color,
            custom_size: Some(Vec2::new(remaining_health, 3.0)),
            ..default()
        },
        transform: Transform::from_xyz(x_pos, 20.0, 0.0),
        ..default()
    }
}
