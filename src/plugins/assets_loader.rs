use std::collections::HashMap;

use bevy::prelude::*;

pub enum AnimationType {
    Idle,
    MovingLeft,
    MovingRight,
}

#[derive(Resource)]
pub struct AssetsLoader {
    pub player_textures: HashMap<AnimationType, AnimationCollection>,
}

#[derive(Component)]
pub struct AnimationCollection {
    pub timer: Timer,
    pub len: i32,
}
