mod animation;
mod constants;
mod player_movement;

use self::{
    animation::animate,
    constants::{ALLOWED_JUMPS, PLAYER_STARING_HP},
    player_movement::movement,
};
use super::asset_loader::{AnimationKey, AnimationMap};
use crate::{
    bundles::character::Character,
    components::{Jumps, Player},
};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, movement)
            .add_systems(Update, animate);
    }
}

fn spawn_player(mut commands: Commands, asset_loader: Res<AnimationMap>) {
    let mut char = (
        Character::new(PLAYER_STARING_HP, Vec2::splat(30.0)),
        Player,
        Jumps(ALLOWED_JUMPS),
    );

    let animation = asset_loader
        .0
        .get(&AnimationKey::Player)
        .expect("Player animation to be found");

    char.0.movable_object.sprite_sheet.texture = animation.texture.clone();
    char.0.movable_object.sprite_sheet.atlas = TextureAtlas {
        layout: animation.atlas.clone(),
        index: 1,
    };

    commands.spawn((char, Name::new("Player")));
}
