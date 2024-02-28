pub mod constants;
pub mod player_movement;

use bevy::prelude::*;

use crate::{
    bundles::character::Character,
    components::{Jumps, Player},
};

use self::constants::*;
use self::player_movement::movement;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, movement);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let char = (
        Character::new(PLAYER_STARING_HP, Vec2::splat(30.0)),
        Player,
        Jumps(ALLOWED_JUMPS),
    );

    let model: Handle<Image> = asset_server.load("knight/all.png");

    commands.spawn((char, Name::new("Player")));
}
