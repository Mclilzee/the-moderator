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

impl AnimationCollection {
    fn new(len: i32, interval: f32) -> Self {
        Self {
            len,
            timer: Timer::new(interval, TimerMode::Repeating),
        }
    }
}

pub struct AssetsLoaderPlugin;
impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets);
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut loader: ResMut<AssetsLoader>,
) {
    loader.player_textures = HashMap::new();

    let texture: Handle<Image> = asset_server.load("knight/idle.png");
    let texture_atlas = texture_atlases.add(TextureAtlas::from_grid(
        texture,
        Vec2::new(21.0, 38.0),
        10,
        1,
        Some(Vec2::new(99.0, 0.0)),
        None,
    ));

    loader
        .player_texture
        .set(AnimationType::Idle, AnimationCollection::new(10, 0.2));
}
