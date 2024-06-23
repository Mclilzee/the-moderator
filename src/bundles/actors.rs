use crate::components::Health;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct Actor {
    collider: Collider,
    hp: Health,
    body: RigidBody,
    pub vel: Velocity,
}

impl Actor {
    pub fn new(hp: i32, width: f32, height: f32) -> Self {
        Self {
            collider: Collider::cuboid(width, height),
            hp: Health(hp),
            body: RigidBody::Dynamic,
            vel: Velocity::zero(),
        }
    }
}
