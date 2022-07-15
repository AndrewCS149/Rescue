use bevy::prelude::*;

use crate::components::{BoundaryTrigger, Collider, Collision, Companion, EntitySize, Speed};

const SIZE_X: f32 = 40.0;
const SIZE_Y: f32 = 40.0;
const SPEED: f32 = 150.0;

pub struct CompanionPlugin;

impl Plugin for CompanionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn);
    }
}

fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
    let companion = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE_X, SIZE_Y)),
            ..default()
        },
        transform: Transform::from_xyz(-200.0, -200.0, 1.0),
        texture: assets.load("dog.png"),
        ..default()
    };

    commands
        .spawn_bundle(companion)
        .insert(Companion)
        .insert(Collider)
        .insert(EntitySize {
            x: SIZE_X,
            y: SIZE_Y,
        })
        .insert(BoundaryTrigger(100.0))
        .insert(Speed(SPEED))
        .insert(Collision::None);
}
