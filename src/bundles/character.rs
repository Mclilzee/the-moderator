use super::movable_object::MovableObject;
use crate::components::{HitBox, Hp};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct Character {
    pub movable_object: MovableObject,
    pub hp: Hp,
    pub hitbox: HitBox,
}

impl Character {
    pub fn new(hp_value: i32, hitbox: Vec2) -> Self {
        Self {
            hp: Hp(hp_value),
            hitbox: HitBox(hitbox),
            ..default()
        }
    }
}
