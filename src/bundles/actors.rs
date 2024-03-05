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
    pub fn new(hp: i32, size: Vec2, state: EntityState) -> Self {
        Self {
            movable_object: MovableObject::default(),
            collider: Collider {
                size,
                collider_type: ColliderType::HitBox { hp },
            },
            state,
        }
    }
}
