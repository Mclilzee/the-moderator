use super::movable_object::MovableObject;
use crate::{
    components::{Collider, ColliderType},
    plugins::physics::entity_type::EntityType,
};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Actor {
    pub movable_object: MovableObject,
    pub collider: Collider,
    pub entity_type: EntityType,
    pub hitbox: HitBox,
}

impl Actor {
    fn new(entity_type: EntityType, hp: i32, size: Vec2) -> Self {
        Self {
            movable_object: MovableObject::default(),
            collider: Collider { size },
            entity_type,
        }
    }
    pub fn grounded(hp: i32, size: Vec2) -> Self {
        Self::new(EntityType::Grounded, hp, size)
    }
}
