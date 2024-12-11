use crate::{
    common_components::EntityState,
    plugins::asset_loader::{AnimationKey, AnimationMap},
};
use bevy::sprite::TextureAtlas;

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

    atlas.index += 1;
    if atlas.index >= frames.last_frame || atlas.index < frames.first_frame {
        atlas.index = frames.first_frame;
    }
}
