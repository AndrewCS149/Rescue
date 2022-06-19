use crate::components::{Direction, Player};
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_projectile::<Player>);
    }
}

fn spawn_projectile<T: Component>(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    query: Query<(&Transform, &Direction), With<T>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let pos = query.single().0.translation;
        let direction = query.single().1;

        let sprite = SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..default()
        };

        commands.spawn_bundle(sprite).insert(*direction);
    }
}
