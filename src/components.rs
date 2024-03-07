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
pub struct AvailableJumps(pub u8);

#[derive(Component, PartialEq, Debug)]
pub enum EntityType {
    Grounded(GroundedState),
    Flying(FlyingState),
    Solid,
}

#[derive(Component, PartialEq, Debug)]
pub enum GroundedState {
    Falling,
    Standing,
    Jumping,
}

#[derive(Component, PartialEq, Debug)]
pub enum FlyingState {
    Ascending,
    Descending,
    Hovering,
}
