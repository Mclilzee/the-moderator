use bevy::prelude::*;

#[derive(Component)]
pub struct MaxJumps(pub u16);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Spammer;

#[derive(Component)]
pub struct Platform;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}
