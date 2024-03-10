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
        Vec2::new(31.0, 38.0),
        18,
        1,
        None,
        None,
    ));

    let first = 1;
    let last = 7;

    let mut range: HashMap<EntityState, AnimationIndices> = HashMap::new();
    range.insert(EntityState::Idle, AnimationIndices::new(first, last));
    range.insert(EntityState::Running, AnimationIndices::new(8, 17));

    let range = Animation {
        texture,
        atlas: layout,
        indices: range,
        default: AnimationIndices::new(first, last),
    };

    animations.0.insert(AnimationKey::Player, range);
}
