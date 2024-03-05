use bevy::prelude::*;

use crate::consts::GRAVITY_ACCELERATION;
use crate::{components::Velocity, plugins::physics::entity_type::EntityType};

use self::constants::{PLAYER_JUMP_HEIGHT, PLAYER_SPEED};

use super::constants::ALLOWED_JUMPS;
use super::*;

pub fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut MaxJumps, &EntityType), With<Player>>,
) {
    let (mut velocity, mut jumps, state) = query.single_mut();
    velocity.0.x = 0.0;
    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        velocity.0.x = PLAYER_SPEED;
    }

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        velocity.0.x = -PLAYER_SPEED;
    }

    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::Space]) {
        if let EntityType::Grounded = *state {
            jumps.0 = ALLOWED_JUMPS
        };

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
