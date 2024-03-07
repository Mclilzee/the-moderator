use bevy::prelude::*;

#[derive(Component)]
pub struct MaxJumps(pub u16);

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

#[derive(Component, PartialEq)]
pub enum EntityState {
    Grounded,
    Flying,
    Solid,
    Airborn,
    Dead,
}
