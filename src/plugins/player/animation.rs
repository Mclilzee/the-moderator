use bevy::prelude::*;

use crate::{
    components::{Player, Velocity},
    AnimationTimer,
};

pub fn animate(
    mut sprite_query: Query<(&Velocity, &mut TextureAtlas, &mut Sprite), With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
) {
    let (velocity, mut atlas, mut sprite) = sprite_query.single_mut();

    let first_frame;
    let last_frame;

    if velocity.translation.x != 0.0 {
        sprite.flip_x = keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]);
        first_frame = 8;
        last_frame = 17;
    } else {
        first_frame = 1;
        last_frame = 7;
    }

    timer.0.tick(time.delta());
    if timer.0.finished() {
        let mut index = atlas.index + 1;

        if atlas.index >= last_frame || atlas.index < first_frame {
            index = first_frame;
        }

        atlas.index = index;
        info!(
            "First Frame {}, Last Frame {}, Index {}",
            first_frame, last_frame, atlas.index
        );
    }
}
