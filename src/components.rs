use bevy::prelude::*;

#[derive(Component)]
pub struct Arrow;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndexRange(pub usize, pub usize);

// describes which action the host is currently doing
#[derive(Component, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Action {
    Interact,
    Idle,
    MeleeAttack,
    RangedAttack,
    Walk,
    Sprint,
}

#[derive(Component, Copy, Clone)]
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
    WalkUpLeft,
    WalkUpRight,
    WalkDownLeft,
    WalkDownRight,
}

#[derive(Component)]
pub struct BoundaryTrigger(pub f32);

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Companion;

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
