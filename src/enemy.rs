use std::time::Duration;

use bevy::prelude::*;

use crate::components::{
    BoundaryTrigger, Collider, Collision, Enemy, EntitySize, Health, Hurting, IsMeleeRange, Speed,
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
        .insert(Health {
            total: HEALTH,
            current: HEALTH,
        })
        .insert(IsMeleeRange(false))
        .insert(BoundaryTrigger(BOUNDARY_TRIGGER));
}

// despawns the enemy if health is at or below zero
fn death(mut commands: Commands, enemy: Query<(Entity, &Health), With<Enemy>>) {
    for (enemy, health) in enemy.iter() {
        if health.current <= 0.0 {
            commands.entity(enemy).despawn_recursive();
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
