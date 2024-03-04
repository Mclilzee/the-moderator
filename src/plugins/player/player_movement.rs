use bevy::prelude::*;

use crate::components::Velocity;
use crate::consts::GRAVITY_ACCELERATION;

use self::constants::{PLAYER_JUMP_HEIGHT, PLAYER_SPEED};

use super::*;

pub fn movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Jumps), With<Player>>,
) {
    let (mut velocity, mut jumps) = query.single_mut();
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        velocity.0.x = PLAYER_SPEED;
    }

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        velocity.0.x = -PLAYER_SPEED;
    }

    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::Space]) && jumps.current >= 1 {
        velocity.0.y = PLAYER_JUMP_HEIGHT + GRAVITY_ACCELERATION;
        jumps.current -= 1;
    }

    if keys.pressed(KeyCode::KeyW) {
        velocity.0.y = PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::KeyS) {
        velocity.0.y = -PLAYER_SPEED;
    }
}
