use bevy::prelude::*;

#[derive(Component)]
pub struct Arrow;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndexRange(pub usize, pub usize);

#[derive(Component)]
pub enum Animation {
    MeleeRight,
    MeleeLeft,
    // MeleeUp,
    // MeleeDown,
    ShootUp,
    ShootDown,
    ShootLeft,
    ShootRight,
    WalkLeft,
    WalkRight,
    WalkUp,
    WalkDown,
}

#[derive(Component)]
pub struct BoundaryTrigger(pub f32);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Damage(pub f32);

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
pub struct Health(pub f32);

#[derive(Component)]
pub struct IsAttacking(pub bool);

#[derive(Component)]
pub struct IsMoving(pub bool);

#[derive(Component)]
pub struct IsSprinting(pub bool);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct EntitySize {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Sprint(pub f32);
