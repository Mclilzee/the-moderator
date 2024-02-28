pub mod constants;
pub mod player_animation;
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

fn spawn_player(mut commands: Commands) {
    let char = (
        Character::new(PLAYER_STARING_HP, Vec2::splat(30.0)),
        Player,
        Jumps(ALLOWED_JUMPS),
    );

    commands.spawn((char, Name::new("Player")));
}
