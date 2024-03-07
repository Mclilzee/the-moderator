use self::constants::{PLAYER_JUMP_HEIGHT, PLAYER_SPEED};
use super::constants::ALLOWED_JUMPS;
use super::*;
use crate::components::{EntityType, Velocity};
use crate::consts::GRAVITY_ACCELERATION;
use bevy::prelude::*;

pub fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity), With<Player>>,
) {
    let (mut velocity, mut jumps) = query.single_mut();
    velocity.0.x = 0.0;
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        velocity.0.x = PLAYER_SPEED;
    }

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        velocity.0.x = -PLAYER_SPEED;
    }

    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::Space]) {
        if jumps.0 > 0 {
            velocity.0.y = PLAYER_JUMP_HEIGHT + GRAVITY_ACCELERATION;
            jumps.0 -= 1;
        }
    }

    if keys.pressed(KeyCode::KeyW) {
        velocity.0.y = PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::KeyS) {
        velocity.0.y = -PLAYER_SPEED;
    }
}
