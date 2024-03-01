use bevy::prelude::*;

use crate::{
    components::Velocity,
    consts::{GRAVITY_ACCELERATION, GRAVITY_MAX_SPEED},
    AnimationTimer,
};

use super::*;

pub fn movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Jumps, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_velocity, mut available_jumps, mut transform) = query.single_mut();
    let mut velocity = Vec3::new(0.0, player_velocity.translation.y, 0.0);
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        velocity.x = PLAYER_SPEED;
    }

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        velocity.x = -PLAYER_SPEED;
    }

    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::Space])
        && available_jumps.0 >= 1
    {
        velocity.y = PLAYER_JUMP_HEIGHT + GRAVITY_ACCELERATION;
        available_jumps.0 -= 1;
    }

    velocity.y -= GRAVITY_ACCELERATION;
    if velocity.y == GRAVITY_MAX_SPEED {
        velocity.y = GRAVITY_MAX_SPEED;
    }

    player_velocity.translation = velocity;

    transform.translation += player_velocity.translation * time.delta_seconds();

    if transform.translation.y < 0.0 {
        player_velocity.translation.y = 0.0;
        transform.translation.y = 0.0;
        available_jumps.0 = ALLOWED_JUMPS;
    }
}

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
