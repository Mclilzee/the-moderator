use bevy::{prelude::*, utils::HashMap};
use std::ops::Range;

use crate::components::Velocity;

#[derive(Bundle, Default)]
pub struct MovableObject {
    pub velocity: Velocity,
    pub sprite_sheet: SpriteSheetBundle,
    pub animation_map: AnimationMap,
}

#[derive(Component, Default)]
pub struct AnimationMap(pub HashMap<Animation, Range<u32>>);

pub enum Animation {
    Idle,
    Running,
}
