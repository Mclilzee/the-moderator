use crate::components::{AvailableJumps, EntityState, Grounded, Player, Velocity};
use bevy::prelude::*;

use super::constants::{PLAYER_JUMP_HEIGHT, PLAYER_MAX_JUMPS, PLAYER_SPEED};

pub fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Grounded,
            &mut Velocity,
            &mut AvailableJumps,
            &mut EntityState,
        ),
        With<Player>,
    >,
) {
    let (mut grounded, mut velocity, mut jumps, mut state) =
        query.get_single_mut().expect("Player should exist");
    velocity.0.x = 0.0;
    *state = EntityState::Idle;
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        *state = EntityState::Running;
        velocity.0.x = PLAYER_SPEED;
    }

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        *state = EntityState::Running;
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
