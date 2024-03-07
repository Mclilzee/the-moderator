use bevy::prelude::*;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Spammer;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider(pub Vec2);

#[derive(Component)]
pub struct Jumps {
    pub current: u8,
    pub max: u8,
}

#[derive(Component, PartialEq, Debug)]
pub enum EntityType {
    Grounded,
    Falling,
    Jumping,
    Flying,
    Solid,
}
