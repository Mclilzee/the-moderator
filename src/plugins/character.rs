use bevy::prelude::*;

use crate::components::{Hp, Velocity};

#[derive(Component)]
pub struct Character {
    pub sprite: SpriteBundle,
    pub hp: Hp,
    pub velocity: Velocity,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            sprite: SpriteBundle {
                transform: Transform::default(),
                visibility: Visibility::Visible,
                ..default()
            },
            hp: Hp(1),
            velocity: Velocity {
                value: Vec2 { x: 50., y: 0. },
            },
        }
    }
}
