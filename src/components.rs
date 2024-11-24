use bevy::{prelude::*, time::Stopwatch};

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Spammer;

#[derive(Component)]
pub struct SpammerDespawnEffect;

#[derive(Component)]
pub struct AvailableJumps(pub u8);

#[derive(Component)]
pub struct Jumps {
    pub current: u8,
    pub max: u8,
}

#[derive(Component)]
pub struct Solid;

#[derive(Eq, Hash, PartialEq, Component)]
pub enum EntityState {
    Idle,
    Running,
    Jumping,
}

#[derive(Component, Default)]
pub struct DespawnStopwatch(pub Stopwatch);
