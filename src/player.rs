use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
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

    commands.spawn_bundle(sprite);
}
