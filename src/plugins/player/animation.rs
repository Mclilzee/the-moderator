use bevy::prelude::*;

use crate::{components::Player, AnimationTimer};

pub fn animate(
    mut atlas_query: Query<&mut TextureAtlas, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
) {
    let mut atlas = atlas_query.single_mut();
    timer.0.tick(time.delta());
    if timer.0.finished() {
        atlas.index = ((atlas.index + 1) % 7) + 1;
    }
}
