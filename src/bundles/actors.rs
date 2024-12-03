use crate::common_components::Health;
use avian2d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct Actor {
    collider: Collider,
    hp: Health,
    body: RigidBody,
    pub sprite_bundle: SpriteBundle,
    pub atlas: TextureAtlas,
}

impl Actor {
    pub fn new(hp: i32, width: f32, height: f32) -> Self {
        Self {
            collider: Collider::capsule(width, height),
            hp: Health(hp),
            body: RigidBody::Dynamic,
            ..default()
        }
    }
}
