use crate::components::{
    AnimationIndexRange, AnimationTimer, Direction, IsMoving, IsSprinting, Player, Speed, Sprint,
};
use crate::physics;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(physics::movement::<Player>)
            .add_system(change_animations);
        // .add_system(animate);
    }
}

// #[derive(Component)]
// enum Animations {
//     IdleUp,
//     IdleDown,
//     MoveRight,
//     MoveLeft,
//     MoveUp,
//     MoveDown,
// }

// fn animate(
//     mut commands: Commands,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//     assets: Res<AssetServer>,
//     player_query: Query<(Entity, &IsMoving, &Direction), With<Player>>,
// ) {
//     for (entity, is_moving, direction) in player_query.iter() {
//         if is_moving.0 {
//             match direction {
//                 Direction::Right => {
//                     let image = assets.load("hero/run_x.png");
//                     let atlas = TextureAtlas::from_grid(image, Vec2::new(49.0, 46.0), 4, 1);
//                     let handle = texture_atlases.add(atlas);

//                     let sprite_sheet = SpriteSheetBundle {
//                         texture_atlas: handle,
//                         transform: Transform::from_scale(Vec3::splat(1.5)),
//                         ..default()
//                     };

//                     let child = commands.spawn_bundle(sprite_sheet).id();

//                     commands
//                         .entity(entity)
//                         .push_children(&[child])
//                         .insert(AnimationTimer(Timer::from_seconds(0.1, true)));
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

fn change_animations(
    mut player_query: Query<(&mut AnimationIndexRange, &Direction), With<Player>>,
) {
    for (mut index_range, direction) in player_query.iter_mut() {
        let new_index_range = match direction {
            Direction::Left => (1, 4),
            Direction::Right => (5, 8),
            Direction::Down => (0, 0),
            Direction::Up => (0, 0),
        };

        index_range.0 = new_index_range.0;
        index_range.1 = new_index_range.1;
    }
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("hero/hero.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::new(49.0, 46.0), 5, 2);
    let handle = texture_atlases.add(atlas);

    let sprite_sheet = SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 0,
            // flip_x: todo!(),
            ..default()
        },
        texture_atlas: handle,
        transform: Transform::from_scale(Vec3::splat(1.5)),
        ..default()
    };

    // let sprite = SpriteBundle {
    //     sprite: Sprite {
    //         custom_size: Some(Vec2::new(73.5, 69.0)),
    //         ..default()
    //     },
    //     texture: assets.load("hero/idle.png"),
    //     ..default()
    // };

    commands
        .spawn_bundle(sprite_sheet)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Player)
        .insert(AnimationIndexRange(0, 1))
        .insert(Speed(150.0))
        .insert(Sprint(1.5))
        .insert(Direction::Down)
        .insert(IsSprinting(false))
        .insert(IsMoving(false));
}
