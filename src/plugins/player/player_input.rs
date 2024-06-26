use crate::components::{AvailableJumps, EntityState, Player};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use super::constants::{PLAYER_JUMP_HEIGHT, PLAYER_SPEED};

pub fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut AvailableJumps, &mut EntityState), With<Player>>,
) {
    let (mut velocity, mut jumps, mut state) = query.get_single_mut().expect("Player should exist");
    velocity.linvel.x = 0.0;
    *state = EntityState::Idle;
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        *state = EntityState::Running;
        velocity.linvel.x = PLAYER_SPEED;
    }

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        *state = EntityState::Running;
        velocity.linvel.x = -PLAYER_SPEED;
    }

    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::Space]) && jumps.0 > 0 {
        velocity.linvel.y = PLAYER_JUMP_HEIGHT;
        jumps.0 -= 1;
    }

    if keys.pressed(KeyCode::KeyW) {
        velocity.linvel.y = PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::KeyS) {
        velocity.linvel.y = -PLAYER_SPEED;
    }
}
