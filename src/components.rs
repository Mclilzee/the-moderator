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

pub enum ColliderType {
    HitBox { hp: i32 },
    HurtBox { dmg: u32 },
    Solid,
}

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
    pub collider_type: ColliderType,
}

impl Collider {
    pub fn new(size: Vec2, collider_type: ColliderType) -> Self {
        Self {
            size,
            collider_type,
        }
    }
}
