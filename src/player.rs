use crate::components::{Direction, Player, Speed, Sprint};
use crate::physics;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(physics::movement::<Player>);
    }
}

fn spawn_player(mut commands: Commands) {
    let sprite = SpriteBundle {
        sprite: Sprite {
            color: Color::ORANGE,
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        ..default()
    };

    commands
        .spawn_bundle(sprite)
        .insert(Player)
        .insert(Speed(150.0))
        .insert(Sprint(1.5))
        .insert(Direction::Down);
}
