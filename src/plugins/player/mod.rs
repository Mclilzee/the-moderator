pub mod constants;
pub mod player_movement;

use bevy::prelude::*;

use crate::{
    bundles::character::Character,
    components::{Jumps, Player},
};

use self::constants::*;
use self::player_movement::movement;

use super::animation_loader::{AnimationKey, AnimationMap};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, movement);
    }
}

fn spawn_player(mut commands: Commands, animation_map: Res<AnimationMap>) {
    let mut char = (
        Character::new(PLAYER_STARING_HP, Vec2::splat(30.0)),
        Player,
        Jumps(ALLOWED_JUMPS),
    );

    let animation = animation_map
        .0
        .get(&AnimationKey::Player)
        .expect("To find player animation");

    char.0.movable_object.sprite_sheet.texture = animation.texture.clone();
    // char.0.movable_object.sprite_sheet.atlas = animation.atlas.clone().atlas;

    commands.spawn((char, Name::new("Player")));
}
