use bevy::prelude::*;

use crate::components::Collider;

#[derive(Bundle)]
pub struct Platform {
    collider: Collider,
    sprite_sheet: SpriteBundle,
}

impl Platform {
    pub fn new(color: Color, size: Vec2) -> Self {
        Self {
            sprite_sheet: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..default()
                },
                ..default()
            },

            collider: Collider(size),
        }
    }
}
