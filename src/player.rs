use crate::components::{Player, Speed};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
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

    commands
        .spawn_bundle(sprite)
        .insert(Player)
        .insert(Speed(150.0));
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Speed, With<Player>)>,
) {
    for (mut transform, speed, _) in player_query.iter_mut() {
        let mut new_pos = Vec3::new(0.0, 0.0, 0.0);

        // left
        if keys.pressed(KeyCode::A) {
            new_pos.x = -1.0;
        }

        // right
        if keys.pressed(KeyCode::D) {
            new_pos.x = 1.0;
        }

        // up
        if keys.pressed(KeyCode::W) {
            new_pos.y = 1.0;
        }

        // down
        if keys.pressed(KeyCode::S) {
            new_pos.y = -1.0;
        }

        transform.translation += new_pos * speed.0 * time.delta_seconds();
    }
}
