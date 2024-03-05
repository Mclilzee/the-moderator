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

#[derive(Default)]
pub enum BoundaryType {
    #[default]
    HitBox,
    HurtBox,
}

#[derive(Component, Default)]
pub struct HitBox {
    pub boundary: Vec2,
    pub boundary_type: BoundaryType,
}
