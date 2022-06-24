use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(build_map);
    }
}

fn build_map(mut commands: Commands, assets: Res<AssetServer>) {
    let map = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
        texture: assets.load("map.png"),
        ..default()
    };

    commands.spawn_bundle(map);
}
