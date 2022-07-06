use crate::components::{
    Action, Animation, AnimationIndexRange, AnimationTimer, Collider, Direction, EntitySize,
    Player, Speed, Sprint,
};
use bevy::prelude::*;

const SIZE_X: f32 = 36.0;
const SIZE_Y: f32 = 46.0;
const SPEED: f32 = 150.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player);
    }
}

// spawns the player to the game window on load
fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("hero.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::new(SIZE_X, SIZE_Y), 4, 14);
    let handle = texture_atlases.add(atlas);

    let sprite_sheet = SpriteSheetBundle {
        texture_atlas: handle,
        transform: Transform::from_scale(Vec3::splat(1.5)),
        ..default()
    };

    commands
        .spawn_bundle(sprite_sheet)
        .insert(Collider)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Player)
        .insert(AnimationIndexRange(0, 1))
        .insert(Speed(SPEED))
        .insert(EntitySize {
            x: SIZE_X,
            y: SIZE_Y,
        })
        .insert(Sprint(1.5))
        .insert(Direction::Down)
        .insert(Animation::ShootLeft)
        .insert(Action::Idle);
}
