use self::constants::{PLAYER_JUMP_HEIGHT, PLAYER_SPEED};
use super::*;
use crate::components::{Jumps, Velocity};
use bevy::prelude::*;

pub fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Jumps), With<Player>>,
) {
    let (mut velocity, mut jumps) = query.get_single_mut().expect("Player should exist");
    velocity.0.x = 0.0;
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        velocity.0.x = PLAYER_SPEED;
    }

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        velocity.0.x = -PLAYER_SPEED;
    }

    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::Space]) && jumps.current > 0 {
        velocity.0.y = PLAYER_JUMP_HEIGHT;
        jumps.current -= 1;
    }

    if keys.pressed(KeyCode::KeyW) {
        velocity.0.y = PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::KeyS) {
        velocity.0.y = -PLAYER_SPEED;
    }
}
