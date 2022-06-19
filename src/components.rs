use bevy::prelude::*;

#[derive(Component, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Sprint(pub f32);

#[derive(Component)]
pub struct Player;
