use bevy::prelude::*;

use crate::{
    common_components::EntityState,
    plugins::asset_loader::{AnimationKey, AnimationMap},
    AnimationTimer,
};

use super::Player;

pub fn animate(
    mut sprite_query: Query<(&mut TextureAtlas, &mut Sprite, &EntityState), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    animation_timer: Res<AnimationTimer>,
    animation: Res<AnimationMap>,
) {
    let (mut atlas, mut sprite, state) = sprite_query.single_mut();

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        sprite.flip_x = true;
    } else if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        sprite.flip_x = false;
    }

    if animation_timer.0.finished() {
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
}
