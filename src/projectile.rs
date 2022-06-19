use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {}
}

fn spawn_projectile(mut commands: Commands) {
    let sprite = SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(5.0, 5.0)),
            ..default()
        },
        ..default()
    };
}
