use super::movable_object::MovableObject;
use crate::components::EntityState;
use crate::components::{Collider, Health};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Actor {
    pub movable_object: MovableObject,
    pub collider: Collider,
    pub hp: Health,
    pub entity_state: EntityState,
}

impl Actor {
    fn new(entity_state: EntityState, hp: i32, size: Vec2) -> Self {
        Self {
            movable_object: MovableObject::default(),
            collider: Collider(size),
            hp: Health(hp),
            entity_state,
        }
    }

    pub fn grounded(hp: i32, size: Vec2) -> Self {
        Self::new(EntityState::Grounded, hp, size)
    }

    pub fn flying(hp: i32, size: Vec2) -> Self {
        Self::new(EntityState::Flying, hp, size)
    }
}
