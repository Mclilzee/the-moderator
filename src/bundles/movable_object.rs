use bevy::{prelude::*, utils::HashMap};
use std::ops::Range;

use crate::components::Velocity;

#[derive(Bundle, Default)]
pub struct MovableObject {
    pub velocity: Velocity,
    pub sprite_sheet: SpriteSheetBundle,
}
