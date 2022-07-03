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

        new_pos.x = calc_new_pos(other_pos.x, player_pos.x);
        new_pos.y = calc_new_pos(other_pos.y, player_pos.y);

        new_pos
    }
}
