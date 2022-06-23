use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndexRange(pub usize, pub usize);

#[derive(Component)]
pub enum Animation {
    // MeleeRight,
    // MeleeLeft,
    // MeleeUp,
    // MeleeDown,
    ShootUp,
    ShootDown,
    ShootLeft,
    ShootRight,
}

#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct IsAttacking(pub bool);

#[derive(Component)]
pub struct IsMoving(pub bool);

#[derive(Component)]
pub struct IsSprinting(pub bool);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Sprint(pub f32);
