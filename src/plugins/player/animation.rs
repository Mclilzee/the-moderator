use bevy::prelude::*;

use crate::{
    components::{EntityState, Player, Velocity},
    plugins::asset_loader::{AnimationKey, AnimationMap},
    AnimationTimer,
};

pub fn animate(
    mut sprite_query: Query<
        (&Velocity, &mut TextureAtlas, &mut Sprite, &EntityState),
        With<Player>,
    >,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
    animation: Res<AnimationMap>,
) {
    let (velocity, mut atlas, mut sprite, state) = sprite_query.single_mut();

    let mut first_frame = 0;
    let mut last_frame = 0;

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        sprite.flip_x = true;
    } else if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        sprite.flip_x = false;
    }

    let player_animations = &animation
        .0
        .get(&AnimationKey::Player)
        .expect("Animation to be found")
        .range;

    if let Some(indices) = player_animations.get(state) {
        first_frame = indices.first_frame;
        last_frame = indices.last_frame;
    }

    // if velocity.0.x.abs() > 0.0 {
    //     first_frame = 8;
    //     last_frame = 17;
    // } else {
    //     first_frame = 1;
    //     last_frame = 7;
    // }

    timer.0.tick(time.delta());
    if timer.0.finished() {
        let mut index = atlas.index + 1;

        if atlas.index >= last_frame || atlas.index < first_frame {
            index = first_frame;
        }

        atlas.index = index;
    }
}
