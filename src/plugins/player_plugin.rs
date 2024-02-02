use bevy::prelude::*;

use crate::components::Hp;

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    sprite: SpriteBundle,
    player: Player,
    hp: Hp,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        sprite: SpriteBundle {
            transform: Transform::default(),
            visibility: Visibility::Visible,
            ..default()
        },
        player: Player,
        hp: Hp(100),
    });
}
