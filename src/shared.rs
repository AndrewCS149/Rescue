// this module is a shared ai module. All AI logic files will share functions within this file

pub mod ai {
    use bevy::prelude::*;
    pub fn tracking(other_pos: Vec3, player_pos: Vec3) -> Vec3 {
        let mut new_pos = Vec3::new(0.0, 0.0, 0.0);
        let buff = 0.25;

        let calc_new_pos = |e_pos: f32, p_pos: f32| match p_pos {
            num if e_pos > num + buff => -1.0,
            num if e_pos < num - buff => 1.0,
            _ => 0.0,
        };

        // if the x difference is greater than the y difference
        if (other_pos.x - player_pos.x).abs() > (other_pos.y - player_pos.y).abs() {
            new_pos.x = calc_new_pos(other_pos.x, player_pos.x);
        } else {
            new_pos.y = calc_new_pos(other_pos.y, player_pos.y);
        }

        new_pos
    }
}

pub mod health {
    use bevy::prelude::*;

    // calculates new health bar size
    pub fn update_healthbar(
        enemy_width: f32,
        current_health: f32,
        original_health: f32,
    ) -> SpriteBundle {
        // compute new healthbar size, color and location
        let remaining_health = enemy_width / (original_health / current_health);
        let x_pos = -((remaining_health - enemy_width).abs() / 2.0);
        let new_color = match current_health {
            h if h < original_health / 4.0 => Color::ORANGE,
            h if h < original_health / 2.0 => Color::YELLOW,
            _ => Color::GREEN,
        };

        // create a new healthbar with updated health color and location
        SpriteBundle {
            sprite: Sprite {
                color: new_color,
                custom_size: Some(Vec2::new(remaining_health, 3.0)),
                ..default()
            },
            transform: Transform::from_xyz(x_pos, 20.0, 0.0),
            ..default()
        }
    }
}
