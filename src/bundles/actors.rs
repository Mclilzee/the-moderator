use super::movable_object::MovableObject;
use crate::{
    components::{Collider, ColliderType, Hp},
    plugins::physics::state::EntityState,
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Actor {
    pub movable_object: MovableObject,
    pub collider: Collider,
    pub state: EntityState,
}

impl Actor {
    pub fn new(hp_value: i32, boundary: Collider) -> Self {
        Self {
            movable_object: MovableObject::default(),
            collider: Collider,
            state: EntityState::Grounded,
        }
    }

    pub fn state(mut self, state: EntityState) -> Self {
        self.state = state;
        self
    }

    pub fn boundary(mut self, boundary: Collider) -> Self {
        self.boundary = boundary;
        self
    }
}
