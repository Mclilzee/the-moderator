use super::character::Character;
use crate::components::{Hp, Player, Velocity};
use bevy::prelude::*;

#[derive(Bundle)]
struct PlayerBundle {
    character: Character,
    player: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            character: Character {
                hp: Hp(100),
                ..default()
            },
            player: Player,
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
