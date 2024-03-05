use bevy::prelude::*;

use crate::components::{Collider, EntityState};

#[derive(Bundle)]
pub struct Platform {
    collider: Collider,
    state: EntityState,
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
            state: EntityState::Solid,
        }
    }
}
