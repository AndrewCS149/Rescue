use crate::components::{AnimationTimer, Direction, IsSprinting, Player, Speed, Sprint};
use crate::physics;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(physics::movement::<Player>);
    }
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let image = assets.load("hero/idle.png");
    // let atlas = TextureAtlas::from_grid(image, Vec2::new(35.0, 47.0), 3, 1);
    // let handle = texture_atlases.add(atlas);

    // let sprite_sheet = SpriteSheetBundle {
    //     texture_atlas: handle,
    // transform: Transform::from_scale(Vec3::splat(6.0)),
    // ..default()
    // };

    let sprite = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(52.5, 70.5)),
            ..default()
        },
        texture: assets.load("hero/idle.png"),
        ..default()
    };

    commands
        .spawn_bundle(sprite)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Player)
        .insert(Speed(150.0))
        .insert(Sprint(1.5))
        .insert(Direction::Down)
        .insert(IsSprinting(false));
}
