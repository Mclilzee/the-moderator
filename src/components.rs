use bevy::{ecs::component::Component, prelude::Bundle, sprite::SpriteBundle};

#[derive(Component)]
pub struct Hp(pub i32);

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Spammer;

#[derive(Bundle)]
pub struct Character {
    pub speed: Speed,
    pub sprite: SpriteBundle,
    pub hp: Hp,
}
