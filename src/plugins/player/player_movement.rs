use bevy::prelude::*;

use crate::{
    components::{Jumps, Player, Velocity},
    consts::GRAVITY_SPEED,
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
        velocity.y = PLAYER_JUMP_HEIGHT + GRAVITY_SPEED;
        available_jumps.0 -= 1;
    }

    velocity.y -= GRAVITY_SPEED;
    player_velocity.translation = velocity;

    transform.translation += player_velocity.translation * time.delta_seconds();

    if transform.translation.y < 0.0 {
        player_velocity.translation.y = 0.0;
        transform.translation.y = 0.0;
        available_jumps.0 = ALLOWED_JUMPS;
    }
}
