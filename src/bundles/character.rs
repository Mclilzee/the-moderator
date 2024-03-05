use super::movable_object::MovableObject;
use crate::components::{BoundaryType, HitBox, Hp};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct Actor {
    pub movable_object: MovableObject,
    pub hp: Hp,
    pub collider: HitBox,
}

impl Actor {
    pub fn new(hp_value: i32, boundary: Vec2) -> Self {
        Self {
            hp: Hp(hp_value),
            collider: HitBox {
                boundary,
                boundary_type: BoundaryType::HitBox,
            },
            ..default()
        }
    }
}
