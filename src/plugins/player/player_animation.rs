use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerAnimation {
    atlas: TextureAtlas,
    idle: Animation,
    moving: Animation,
}

pub struct Animation {
    pub first_frame: u32,
    pub last_frame: u32,
}

impl Animation {
    pub fn new(first_frame: u32, last_frame: u32) -> Self {
        Animation {
            first_frame,
            last_frame,
        }
    }
}
