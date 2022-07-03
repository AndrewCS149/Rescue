use bevy::prelude::*;

use crate::components::{Companion, Speed};

const SIZE_X: f32 = 40.0;
const SIZE_Y: f32 = 40.0;

pub struct DogPlugin;

impl Plugin for DogPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_dog);
    }
}

fn spawn_dog(mut commands: Commands, assets: Res<AssetServer>) {
    let dog = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(SIZE_X, SIZE_Y)),
            ..default()
        },
        transform: Transform::from_xyz(-200.0, -200.0, 1.0),
        texture: assets.load("dog.png"),
        ..default()
    };

    commands
        .spawn_bundle(dog)
        .insert(Companion)
        .insert(Speed(100.0));
}
