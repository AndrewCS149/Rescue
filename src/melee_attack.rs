use bevy::prelude::*;

use crate::components::{Action, Animation, Collision, Direction, Enemy, Player};

pub struct MeleeAttackPlugin;

impl Plugin for MeleeAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attack::<Player>);
    }
}

// fn detect_hit(
//     player_query: Query<&Collision, With<Player>>,
//     enemy_query: Query<&Collision, With<Enemy>>,
// ) {
// }

fn attack<T: Component>(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&Direction, &mut Action, &mut Animation), With<T>>,
) {
    for (direction, mut action, mut animation) in player_query.iter_mut() {
        if keys.just_pressed(KeyCode::Space) && *action != Action::MeleeAttack {
            *action = Action::MeleeAttack;

            *animation = match direction {
                Direction::Left => Animation::MeleeLeft,
                Direction::Right => Animation::MeleeRight,
                Direction::Up => Animation::MeleeUp,
                Direction::Down => Animation::MeleeDown,
            };
        }
    }
}

// calculates new health bar size
// fn update_healthbar(enemy_width: f32, health: f32) -> SpriteBundle {
//     // compute new healthbar size, color and location
//     let remaining_health = enemy_width / (HEALTH / health);
//     let x_pos = -((remaining_health - enemy_width).abs() / 2.0);
//     let new_color = match health {
//         h if h < HEALTH / 4.0 => Color::ORANGE,
//         h if h < HEALTH / 2.0 => Color::YELLOW,
//         _ => Color::GREEN,
//     };

//     // create a new healthbar with updated health color and location
//     SpriteBundle {
//         sprite: Sprite {
//             color: new_color,
//             custom_size: Some(Vec2::new(remaining_health, 3.0)),
//             ..default()
//         },
//         transform: Transform::from_xyz(x_pos, 20.0, 0.0),
//         ..default()
//     }
// }
