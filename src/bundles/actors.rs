use crate::components::Health;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::movable_sprite::MovableSprite;

#[derive(Bundle)]
pub struct Actor {
    pub movable_sprite: MovableSprite,
    pub collider: Collider,
    pub hp: Health,
}

impl Actor {
    pub fn new(hp: i32, width: f32, height: f32) -> Self {
        Self {
            movable_sprite: MovableSprite::default(),
            collider: Collider::cuboid(width, height),
            hp: Health(hp),
        }
    }
}
