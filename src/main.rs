use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .run();
}

fn spawn_player(mut commands: Commands) {
    let sprite = SpriteBundle {
        sprite: Sprite {
            color: Color::AZURE,
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        ..default()
    };

    commands.spawn_bundle(sprite);
}
