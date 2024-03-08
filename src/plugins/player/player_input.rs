use crate::components::{AvailableJumps, Grounded, Player, Velocity};
use bevy::prelude::*;

use super::constants::{PLAYER_JUMP_HEIGHT, PLAYER_MAX_JUMPS, PLAYER_SPEED};

pub fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Grounded, &mut Velocity, &mut AvailableJumps), With<Player>>,
) {
    let (mut grounded, mut velocity, mut jumps) =
        query.get_single_mut().expect("Player should exist");
    velocity.0.x = 0.0;
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        velocity.0.x = PLAYER_SPEED;
    }

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        velocity.0.x = -PLAYER_SPEED;
    }

    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::Space]) {
        if grounded.0 {
            jumps.0 = PLAYER_MAX_JUMPS;
        }

        if jumps.0 > 0 {
            velocity.0.y = PLAYER_JUMP_HEIGHT;
            jumps.0 -= 1;
            grounded.0 = false;
        }
    }

    if keys.pressed(KeyCode::KeyW) {
        velocity.0.y = PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::KeyS) {
        velocity.0.y = -PLAYER_SPEED;
    }
}
