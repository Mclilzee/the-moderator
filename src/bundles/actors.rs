use super::movable_object::MovableObject;
use crate::{
    components::{Collider, ColliderType},
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
    pub fn grounded(hp: i32, size: Vec2) -> Self {
        Self {
            movable_object: MovableObject::default(),
            collider: Collider {
                size,
                collider_type: ColliderType::HitBox { hp },
            },
            state: EntityState::Grounded,
        }
    }
}
