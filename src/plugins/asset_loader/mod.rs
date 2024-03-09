mod player_assets;
use bevy::{prelude::*, utils::HashMap};

use crate::components::EntityState;

#[derive(Resource, Default)]
pub struct AnimationMap(pub HashMap<AnimationKey, Animation>);

#[derive(Eq, Hash, PartialEq)]
pub enum AnimationKey {
    Player,
}

pub struct Animation {
    pub texture: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
    pub range: HashMap<EntityState, AnimationIndices>,
}

pub struct AnimationIndices {
    pub first_frame: u32,
    pub last_frame: u32,
}

impl AnimationIndices {
    fn new(first_frame: u32, last_frame: u32) -> Self {
        AnimationIndices {
            first_frame,
            last_frame,
        }
    }
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AnimationMap::default())
            .add_systems(PreStartup, player_assets::setup);
    }
}
