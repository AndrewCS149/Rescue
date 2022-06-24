use crate::components::{
    Animation, AnimationIndexRange, AnimationTimer, Direction, IsAttacking, IsMoving, IsSprinting,
    Player, Speed, Sprint,
};
use bevy::prelude::*;

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
    let atlas = TextureAtlas::from_grid(image, Vec2::new(36.0, 46.0), 4, 8);
    let handle = texture_atlases.add(atlas);

    let sprite_sheet = SpriteSheetBundle {
        texture_atlas: handle,
        transform: Transform::from_scale(Vec3::splat(1.5)),
        ..default()
    };

    commands
        .spawn_bundle(sprite_sheet)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Player)
        .insert(AnimationIndexRange(0, 1))
        .insert(Speed(150.0))
        .insert(Sprint(1.5))
        .insert(Direction::Down)
        .insert(Animation::ShootLeft)
        .insert(IsAttacking(false))
        .insert(IsSprinting(false))
        .insert(IsMoving(false));
}
