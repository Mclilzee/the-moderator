use bevy::prelude::*;

use crate::components::{Collider, EntityType};

#[derive(Bundle)]
pub struct Platform {
    collider: Collider,
    state: EntityType,
    sprite_sheet: SpriteBundle,
}

impl Platform {
    pub fn new(color: Color, size: Vec2) -> Self {
        Self {
            sprite_sheet: SpriteBundle {
                sprite: Sprite {
                    color: Color::PURPLE,
                    custom_size: Some(size),
                    ..default()
                },
                ..default()
            },

            collider: Collider(size),
            state: EntityType::Solid,
        }
    }
}
