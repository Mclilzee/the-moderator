use bevy::sprite::Sprite;
use crate::{common_components::EntityState, plugins::asset_loader::{AnimationKey, AnimationMap}};

pub fn animate(
    sprite: &mut Sprite,
    state: &EntityState,
    key: &AnimationKey,
    map: &AnimationMap,
) {
    let player_animations = map.0.get(key).expect("Animation were not found");
    let frames = player_animations
        .indices
        .get(state)
        .unwrap_or(&player_animations.default);

    let atlas = sprite.texture_atlas.as_mut().unwrap();
    if atlas.index >= frames.last_frame || atlas.index < frames.first_frame {
        atlas.index = frames.first_frame;
    }
}
