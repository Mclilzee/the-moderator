use bevy::prelude::*;

use crate::{
    common_components::EntityState,
    plugins::asset_loader::{AnimationKey, AnimationMap},
};

use super::Player;

pub fn animate(
    mut sprite_query: Query<(&mut TextureAtlas, &EntityState), With<Player>>,
    animation: Res<AnimationMap>,
) {
    let (mut atlas, state) = sprite_query.single_mut();

    let player_animations = &animation
        .0
        .get(&AnimationKey::Player)
        .expect("Animation to be found");

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

pub fn flip_on_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut sprite: Query<&mut Sprite, With<Player>>,
) {
    let mut sprite = sprite.single_mut();

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        sprite.flip_x = true;
    } else if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        sprite.flip_x = false;
    }
}
