use crate::components::{Character, Hp, Player, Velocity};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 160.0;
const PLAYER_JUMP_HEIGHT: f32 = 10.0;
const PLAYER_STARING_HP: i32 = 100;
const PLAYER_WIDTH: f32 = 30.0;
const PLAYER_HEIGHT: f32 = 50.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_input, move_player).chain());
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
            velocity: Velocity(Vec2::ZERO),
        },
        Player,
    ));
}

fn player_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
    time: Res<Time>,
) {
    let mut velocity = query.single_mut();
    if keys.pressed(KeyCode::Right) {
        velocity.0.x += PLAYER_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::Left) {
        velocity.0.x -= PLAYER_SPEED * time.delta_seconds();
    }

    if keys.pressed(KeyCode::Up) {
        velocity.0.y += PLAYER_JUMP_HEIGHT * time.delta_seconds();
    }
}

fn move_player(mut player_query: Query<(&mut Transform, &Velocity), With<Player>>) {
    let (mut transform, velocity) = player_query.single_mut();

    transform.translation += velocity.0.extend(0.0);
}
