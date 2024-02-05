use crate::{
    components::{Character, Hp, Player, Velocity},
    consts::GRAVITY_SPEED,
};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 160.0;
const PLAYER_JUMP_HEIGHT: f32 = 500.0;
const PLAYER_STARING_HP: i32 = 100;
const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 40.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, movement);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Character {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                    ..default()
                },
                ..default()
            },
            hp: Hp(PLAYER_STARING_HP),
            velocity: Velocity::default(),
        },
        Player,
    ));
}

fn movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_transform, mut player_velocity) = query.single_mut();
    let mut velocity = Vec3::new(0.0, player_velocity.translation.y, 0.0);

    if keys.pressed(KeyCode::Right) {
        velocity.x = PLAYER_SPEED;
    }

    if keys.pressed(KeyCode::Left) {
        velocity.x = -PLAYER_SPEED;
    }

    if keys.just_pressed(KeyCode::Up) && player_velocity.translation.y == 0.0 {
        velocity.y = PLAYER_JUMP_HEIGHT + GRAVITY_SPEED;
    }

    velocity.y -= GRAVITY_SPEED;
    player_velocity.translation = velocity;

    player_transform.translation += player_velocity.translation * time.delta_seconds();

    if player_transform.translation.y < 0.0 {
        player_velocity.translation.y = 0.0;
        player_transform.translation.y = 0.0;
    }
}
