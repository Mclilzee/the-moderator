mod player_assets;
use bevy::{prelude::*, utils::HashMap};

use crate::components::EntityState;

#[derive(Resource, Default)]
pub struct AnimationMap(pub HashMap<AnimationKey, Animation>);

#[derive(Eq, Hash, PartialEq)]
pub enum AnimationKey {
    Player,
    Spammer,
}

pub struct Animation {
    pub texture: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
    pub indices: HashMap<EntityState, AnimationIndices>,
    pub default: AnimationIndices,
}

pub struct AnimationIndices {
    pub first_frame: usize,
    pub last_frame: usize,
}

impl AnimationIndices {
    fn new(first_frame: usize, last_frame: usize) -> Self {
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
