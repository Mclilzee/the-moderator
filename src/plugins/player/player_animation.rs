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

pub fn load_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlasLayout>>,
) {
    let atlas_handle: Handle<Image> = asset_server.load("knight/idle.png");
    let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(32, 32), 7, 4, None, None));
}
