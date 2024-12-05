use crate::common_components::Health;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle, Default)]
pub struct Actor {
    collider: Collider,
    hp: Health,
    body: RigidBody,
    pub sprite_bundle: SpriteBundle,
    pub atlas: TextureAtlas,
    pub vel: Velocity,
}

impl Actor {
    pub fn new(hp: i32, height: f32, radius: f32) -> Self {
        Self {
            collider: Collider::capsule_y(height, radius),
            hp: Health(hp),
            body: RigidBody::Dynamic,
            vel: Velocity::zero(),
            ..default()
        }
    }
}
