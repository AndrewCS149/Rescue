use crate::components::{AnimationTimer, Direction, IsMoving, IsSprinting, Player, Speed, Sprint};
use crate::physics;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(physics::movement::<Player>)
            .add_system(animate);
    }
}

fn animate(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    assets: Res<AssetServer>,
    player_query: Query<(&IsMoving, &Direction), With<Player>>,
) {
    for (is_moving, direction) in player_query.iter() {
        if is_moving.0 {
            println!("RUNNING");
        } else {
            println!("NOT RUNNING");
        }
        // let image = assets.load("hero/run_x.png");
        // let atlas = TextureAtlas::from_grid(image, Vec2::new(49.0, 46.0), 4, 1);
        // let handle = texture_atlases.add(atlas);

        // let sprite_sheet = SpriteSheetBundle {
        //     texture_atlas: handle,
        //     transform: Transform::from_scale(Vec3::splat(1.5)),
        //     ..default()
        // };
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
            custom_size: Some(Vec2::new(73.5, 69.0)),
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
        .insert(IsSprinting(false))
        .insert(IsMoving(false));
}
