use super::movable_object::MovableObject;
use crate::{
    components::{BoundaryBox, BoundaryType, Hp},
    plugins::physics::state::EntityState,
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Actor {
    pub movable_object: MovableObject,
    pub hp: Hp,
    pub collider: BoundaryBox,
    pub state: EntityState,
}

impl Actor {
    pub fn new(hp_value: i32, boundary: Vec2) -> Self {
        Self {
            hp: Hp(hp_value),
            collider: BoundaryBox {
                boundary,
                boundary_type: BoundaryType::HitBox,
            },
        }
    }

    pub fn state(mut self, state: EntityState) -> Self {
        self.state = state;
        self
    }

    pub fn hp(mut self, hp: i32) -> Self {
        self.hp = Hp(hp);
        self
    }
}
