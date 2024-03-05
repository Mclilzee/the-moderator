use super::movable_object::MovableObject;
use crate::components::{BoundaryBox, BoundaryType, Hp};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct Actor {
    pub movable_object: MovableObject,
    pub hp: Hp,
    pub collider: BoundaryBox,
}

impl Actor {
    pub fn new(hp_value: i32, boundary: Vec2) -> Self {
        Self {
            hp: Hp(hp_value),
            collider: BoundaryBox {
                boundary,
                boundary_type: BoundaryType::HitBox,
            },
            ..default()
        }
    }
}
