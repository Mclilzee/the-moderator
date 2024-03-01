use bevy::prelude::*;

use crate::{components::Player, AnimationTimer};

pub fn animate(
    mut atlas_query: Query<(&mut TextureAtlas, &mut Sprite), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
) {
    let (mut atlas, mut sprite) = atlas_query.single_mut();
    let first_frame;
    let last_frame;
    if keys.any_pressed([
        KeyCode::ArrowLeft,
        KeyCode::KeyA,
        KeyCode::ArrowRight,
        KeyCode::KeyD,
    ]) {
        first_frame = 8;
        last_frame = 17;
        sprite.flip_x = keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]);
    } else {
        first_frame = 1;
        last_frame = 7;
    }

    timer.0.tick(time.delta());
    if timer.0.finished() {
        let mut index = (atlas.index + 1) % last_frame;

        if index > last_frame || index < first_frame {
            index = first_frame;
        }
        atlas.index = index;
        info!(
            "First Frame {}, Last Frame {}, Index {}",
            first_frame, last_frame, atlas.index
        );
    }
}
