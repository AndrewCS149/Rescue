use bevy::prelude::*;

use crate::components::Enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy);
        //     .add_system(physics::movement::<Player>);
    }
}

fn spawn_enemy(mut commands: Commands) {
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

    commands.spawn_bundle(enemy).insert(Enemy);
}
