use bevy::prelude::*;

#[derive(Component)]
pub struct Jumps(pub i32);

#[derive(Component, Default)]
pub struct Hp(pub i32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Spammer;

#[derive(Component)]
pub struct Platform;

#[derive(Component, Default)]
pub struct Velocity {
    pub translation: Vec3,
}

#[derive(Component, Default)]
pub struct HitBox(pub Vec2);
