use bevy::prelude::*;

use crate::components::Velocity;

#[derive(Bundle, Default)]
pub struct MovableObject {
    pub velocity: Velocity,
    pub sprite_sheet: SpriteSheetBundle,
}
