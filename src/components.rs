use bevy::prelude::*;

#[derive(Component)]
pub struct Jumps {
    pub current: u16,
    pub max: u16,
}

#[derive(Component, Default)]
pub struct Hp(pub i32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Spammer;

#[derive(Component)]
pub struct Platform;

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

pub enum BoundaryType {
    HitBox { hp: i32 },
    HurtBox { dmg: u32 },
}

#[derive(Component)]
pub struct BoundaryBox {
    pub boundary: Vec2,
    pub boundary_type: BoundaryType,
}

impl BoundaryBox {
    pub fn new(boundary: Vec2, boundary_type: BoundaryType) -> Self {
        Self {
            boundary,
            boundary_type,
        }
    }
}
