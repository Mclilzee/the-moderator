use bevy::sprite::TextureAtlas;
use crate::{common_components::EntityState, plugins::asset_loader::{AnimationKey, AnimationMap}};

pub fn animate(
    atlas: &mut TextureAtlas,
    state: &EntityState,
    key: &AnimationKey,
    map: &AnimationMap,
) {
    let player_animations = map.0.get(key).expect("Animation were not found");
    let frames = player_animations
        .indices
        .get(state)
        .unwrap_or(&player_animations.default);

    let mut index = atlas.index + 1;

    if atlas.index >= frames.last_frame || atlas.index < frames.first_frame {
        index = frames.first_frame;
    }

    atlas.index = index;
}
