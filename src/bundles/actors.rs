use super::movable_object::MovableObject;
use crate::components::{Collider, ColliderType, Health};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Actor {
    pub movable_object: MovableObject,
    pub collider: Collider,
    pub hp: Health,
}

impl Actor {
    fn new(collider_type: ColliderType, hp: i32, size: Vec2) -> Self {
        Self {
            movable_object: MovableObject::default(),
            collider: Collider {
                size,
                collider_type,
            },
            hp: Health(hp),
        }
    }
    pub fn dynamic(hp: i32, size: Vec2) -> Self {
        Self::new(ColliderType::Dynamic, hp, size)
    }

    pub fn flying(hp: i32, size: Vec2) -> Self {
        Self::new(ColliderType::Flying, hp, size)
    }
}
