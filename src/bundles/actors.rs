use crate::common_components::Health;
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Collider, Health, RigidBody, Sprite)]
pub struct Actor;

impl Actor {
    pub fn new(hp: i32, width: f32, height: f32) -> (Actor, RigidBody, Collider, Health) {
        (
            Self,
            RigidBody::Dynamic,
            Collider::capsule(width, height),
            Health(hp),
        )
    }
}
