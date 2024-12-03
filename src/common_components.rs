use bevy::prelude::*;
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Friendly;

#[derive(Component, Default)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Component, Default)]
pub enum EntityState {
    #[default]
    Idle,
    Running,
    Jumping,
    Falling,
    DoubleJumping
}
