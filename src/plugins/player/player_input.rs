use crate::common_components::EntityState;
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use super::{Player, PLAYER_JUMP_HEIGHT, PLAYER_SPEED};

pub fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &mut EntityState), With<Player>>,
) {
    let (mut velocity, mut state) = query.get_single_mut().expect("Player should exist");
    velocity.x = 0.0;

    if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        velocity.x = PLAYER_SPEED;
    }

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        velocity.x = -PLAYER_SPEED;
    }

    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::Space])
        && *state != EntityState::DoubleJumping
    {
        velocity.y = PLAYER_JUMP_HEIGHT;
        match *state {
            EntityState::Jumping | EntityState::Falling => *state = EntityState::DoubleJumping,
            _ => *state = EntityState::Jumping,
        }
    }

    if velocity.x != 0.0
        && !matches!(
            *state,
            EntityState::Jumping | EntityState::Falling | EntityState::DoubleJumping
        )
    {
        *state = EntityState::Running;
    }
}

pub fn flip_on_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut sprite: Query<&mut Sprite, With<Player>>,
) {
    let mut sprite = sprite.single_mut();

    if keys.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        sprite.flip_x = true;
    } else if keys.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        sprite.flip_x = false;
    }
}
