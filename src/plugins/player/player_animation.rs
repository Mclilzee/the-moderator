use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerAnimation {
    atlas: TextureAtlas,
}

pub struct Animation {
    first_frame: u32,
    last_frame: u32,
}

impl Animation {
    fn new(first_frame: u32, last_frame: u32) -> Self {
        Animation {
            first_frame,
            last_frame,
        }
    }
}
