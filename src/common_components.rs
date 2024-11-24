use std::time::Duration;

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

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(100), TimerMode::Repeating))
    }
}

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Component)]
pub enum EntityState {
    Idle,
    Running,
    Jumping,
}
