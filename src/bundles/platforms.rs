use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;

#[derive(Bundle)]
pub struct Platforms {
    collider: Collider,
    sprite_sheet: SpriteBundle,
}

impl Platforms {
    pub fn cuboid(color: Color, size: Vec2, transform: Transform) -> Self {
        Self {
            sprite_sheet: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..default()
                },
                transform,
                ..default()
            },
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
        }
    }
}
