mod animation;
mod constants;
mod player_input;

use self::constants::PLAYER_MAX_JUMPS;
use self::{animation::animate, constants::PLAYER_STARING_HP, player_input::input};
use super::asset_loader::AnimationKey;
use super::asset_loader::AnimationMap;
use crate::components::{Grounded, Jumps};
use crate::{
    bundles::actors::Actor,
    components::{AvailableJumps, Damage, Player},
    InGameSet,
};
use bevy::{prelude::*, render::camera::ScalingMode};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, input.in_set(InGameSet::Input))
            .add_systems(Update, animate);
    }
}

fn spawn_player(mut commands: Commands, asset_loader: Res<AnimationMap>) {
    let mut char = (
        Actor::new(PLAYER_STARING_HP, Vec2::new(15.0, 35.0)),
        Player,
        AvailableJumps(PLAYER_MAX_JUMPS),
        Damage(5),
        Jumps {
            current: 20,
            max: 2,
        },
        Grounded(true),
    );

    let animation = asset_loader
        .0
        .get(&AnimationKey::Player)
        .expect("Player animation to be found");

    char.0.movable_sprite.sprite_sheet.texture = animation.texture.clone();
    char.0.movable_sprite.sprite_sheet.atlas = TextureAtlas {
        layout: animation.atlas.clone(),
        index: 1,
    };

    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 500.0,
        min_height: 400.0,
    };

    let player_id = commands.spawn((char, Name::new("Player"))).id();
    let mut camera = commands.spawn(camera);
    camera.set_parent(player_id);
}
