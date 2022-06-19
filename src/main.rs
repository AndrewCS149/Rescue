use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .run();
}

fn setup(mut commands: Commands) {
    let camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);
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
