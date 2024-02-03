use crate::components::{Character, Hp, Player, Speed};
use bevy::prelude::*;

const PLAYER_STARTING_SPEED: f32 = 60.0;
const PLAYER_STARING_HP: i32 = 100;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Character {
            sprite: SpriteBundle::default(),
            hp: Hp(PLAYER_STARING_HP),
            speed: Speed(PLAYER_STARTING_SPEED),
        },
        Player,
    ));
}

fn move_player(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time>,
) {
    let (mut transform, speed) = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::Left) {
        direction.x -= speed.0 * time.delta_seconds();
    }

    if keys.pressed(KeyCode::Right) {
        direction.x += speed.0 * time.delta_seconds();
    }

    if keys.pressed(KeyCode::Up) {
        direction.y += speed.0 * time.delta_seconds();
    }

    if keys.pressed(KeyCode::Down) {
        direction.y -= speed.0 * time.delta_seconds();
    }

    direction = direction.normalize_or_zero();

    transform.translation += direction;
}
