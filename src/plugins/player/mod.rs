mod animation;
mod attacks;
mod constants;
mod player_input;

use self::constants::PLAYER_MAX_JUMPS;
use self::{animation::animate, constants::PLAYER_STARING_HP, player_input::input};
use super::asset_loader::AnimationKey;
use super::asset_loader::AnimationMap;
use crate::components::{EntityState, Jumps};
use crate::{
    bundles::actors::Actor,
    components::{AvailableJumps, Damage, Player},
};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::LockedAxes;
use bevy_rapier2d::geometry::{CollisionGroups, Group};
use constants::{PLAYER_HEIGHT, PLAYER_WIDTH};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(attacks::AttacksPlugin)
            .add_systems(PostStartup, spawn_player)
            .add_systems(Update, input)
            .add_systems(Update, animate);
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_loader: Res<AnimationMap>,
    camera_q: Query<Entity, With<Camera>>,
) {
    let mut char = (
        Actor::new(PLAYER_STARING_HP, PLAYER_WIDTH, PLAYER_HEIGHT),
        CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
        Player,
        AvailableJumps(PLAYER_MAX_JUMPS),
        Damage(5),
        Jumps {
            current: 20,
            max: 2,
        },
        EntityState::Idle,
        LockedAxes::ROTATION_LOCKED,
    );

    let animation = asset_loader
        .0
        .get(&AnimationKey::Player)
        .expect("Player animation were not found");

    char.0.sprite_bundle.texture = animation.texture.clone();
    char.0.atlas = TextureAtlas {
        layout: animation.atlas.clone(),
        index: 1,
    };

    let player_id = commands.spawn((char, Name::new("Player"))).id();
    let id = camera_q.single();
    commands.get_entity(id).unwrap().set_parent(player_id);
}
