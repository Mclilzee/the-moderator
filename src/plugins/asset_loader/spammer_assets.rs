use bevy::{prelude::*, utils::HashMap};

use super::{Animation, AnimationIndices, AnimationKey, AnimationMap};
use crate::components::EntityState;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut atlas_server: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<AnimationMap>,
) {
    let texture: Handle<Image> = asset_server.load("monsters/Spammer.png");
    let layout = atlas_server.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        13,
        1,
        None,
        None,
    ));

    let idle_animation = AnimationIndices::new(1, 10);
    let mut indices: HashMap<EntityState, AnimationIndices> = HashMap::new();
    indices.insert(EntityState::Idle, idle_animation);

    let animation = Animation {
        texture,
        atlas: layout,
        indices,
        default: idle_animation,
    };

    animations.0.insert(AnimationKey::Spammer, animation);
}
