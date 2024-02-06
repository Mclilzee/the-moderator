use super::movable_object::MovableObject;
use crate::components::{HitBox, Hp};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct Character {
    pub movable_object: MovableObject,
    pub sprite_sheet: SpriteSheetBundle,
    pub hp: Hp,
    pub hitbox: HitBox,
}

impl Character {
    pub fn new(hp_value: i32, hitbox: Option<Vec2>) -> Self {
        let mut char = Self {
            hp: Hp(hp_value),
            ..default()
        };

        if let Some(hitbox) = hitbox {
            char.hitbox = HitBox(hitbox);
        }

        char
    }
}
