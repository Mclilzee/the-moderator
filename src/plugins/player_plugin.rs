use bevy::prelude::*;

use crate::components::Hp;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity {
    value: Vec2,
}

#[derive(Bundle)]
struct PlayerBundle {
    sprite: SpriteBundle,
    player: Player,
    hp: Hp,
    velocity: Velocity,
    name: Name,
}

impl PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            sprite: SpriteBundle {
                transform: Transform::default(),
                visibility: Visibility::Visible,
                ..default()
            },
            player: Player,
            hp: Hp(100),
            name: Name::from("Fred"),
            velocity: Velocity {
                value: Vec2 { x: 50., y: 0. },
            },
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

fn move_player(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    let (mut transform, velocity) = query.get_single_mut().expect("Player is not found");

    if keys.pressed(KeyCode::Left) {
        transform.translation.x -= velocity.value.x * time.delta_seconds();
    } else if keys.pressed(KeyCode::Right) {
        transform.translation.x += velocity.value.x * time.delta_seconds();
    }
}
