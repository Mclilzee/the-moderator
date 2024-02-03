use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Hp(pub i32);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Spammer;
