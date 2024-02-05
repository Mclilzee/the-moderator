use bevy::prelude::*;

#[derive(Component)]
pub struct Hp(pub i32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Spammer;

#[derive(Bundle)]
pub struct Character {
    pub velocity: Velocity,
    pub sprite_sheet: SpriteSheetBundle,
    pub hp: Hp,
}

#[derive(Component, Default)]
pub struct Velocity {
    pub translation: Vec3,
}
