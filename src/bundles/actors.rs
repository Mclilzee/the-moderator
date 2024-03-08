use crate::components::{Collider, Health};
use bevy::prelude::*;

use super::movable_sprite::MovableSprite;

#[derive(Bundle)]
pub struct Actor {
    pub movable_sprite: MovableSprite,
    pub collider: Collider,
    pub hp: Health,
}

impl Actor {
    pub fn new(hp: i32, size: Vec2) -> Self {
        Self {
            movable_sprite: MovableSprite::default(),
            collider: Collider(size),
            hp: Health(hp),
        }
    }
}
