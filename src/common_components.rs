use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Jumps {
    pub current: u8,
    pub max: u8,
}

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Component)]
pub enum EntityState {
    Idle,
    Running,
    Jumping,
}
