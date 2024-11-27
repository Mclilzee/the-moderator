use crate::{common_components::Health, plugins::asset_loader::AnimationKey};
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
    pub animation_key: AnimationKey
}

impl Actor {
    pub fn new(hp: i32, width: f32, height: f32, animation_key: AnimationKey) -> Self {
        Self {
            collider: Collider::cuboid(width, height),
            hp: Health(hp),
            body: RigidBody::Dynamic,
            vel: Velocity::zero(),
            animation_key,
            ..default()
        }
    }
}
