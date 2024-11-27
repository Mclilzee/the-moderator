mod animation;
mod attacks;
mod player_input;

use self::{animation::animate, player_input::input};
use super::asset_loader::AnimationKey;
use super::asset_loader::AnimationMap;
use super::platform::Platform;
use crate::common_components::{EntityState, Jumps};
use crate::{bundles::actors::Actor, common_components::Damage};
use bevy::prelude::*;
use bevy_rapier2d::dynamics::LockedAxes;
use bevy_rapier2d::geometry::{CollisionGroups, Group};
use bevy_rapier2d::plugin::RapierContext;

pub const PLAYER_SPEED: f32 = 150.0;
pub const PLAYER_JUMP_HEIGHT: f32 = 300.0;
pub const PLAYER_STARING_HP: i32 = 100;
pub const PLAYER_MAX_JUMPS: u8 = 2;
pub const PLAYER_HEIGHT: f32 = 17.0;
pub const PLAYER_WIDTH: f32 = 7.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(attacks::AttacksPlugin)
            .add_systems(PostStartup, spawn_player)
            .add_systems(Update, input)
            .add_systems(Update, animate)
            .add_systems(Update, ground_collision);
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
        Damage(5),
        Jumps {
            current: 0,
            max: PLAYER_MAX_JUMPS,
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

fn ground_collision(
    mut player: Query<(Entity, &Transform, &mut Jumps), With<Player>>,
    platforms: Query<(Entity, &Transform), With<Platform>>,
    rapier_context: Res<RapierContext>,
) {
    let (p_id, p_transform, mut jumps) = player.single_mut();
    for (platform_id, platform_transform) in platforms.iter() {
        if rapier_context.contact_pair(p_id, platform_id).is_some()
            && p_transform.translation.y > platform_transform.translation.y
        {
            jumps.current = 0;
        }
    }
}
