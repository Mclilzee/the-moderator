use crate::{
    common_components::{Damage, EntityState, Friendly},
    plugins::{
        asset_loader::{AnimationEvent, AnimationKey, AnimationMap},
        player::{JumpEvent, Player, PLAYER_LENGTH},
    },
};

use avian2d::prelude::{Collider, Sensor};
use bevy::prelude::*;

const FIRE_SLASH_WIDTH: f32 = 80.0;
const FIRE_SLASH_HEIGHT: f32 = 10.0;
const DAMAGE: i32 = 10;

#[derive(Component)]
struct FireSlash;

pub struct FireSlashPlugin;

impl Plugin for FireSlashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn.run_if(on_event::<JumpEvent>))
            .add_systems(
                Update,
                animate_then_despawn.run_if(on_event::<AnimationEvent>),
            );
    }
}

fn spawn(
    mut commands: Commands,
    player_transform: Single<&Transform, With<Player>>,
    animation_map: Res<AnimationMap>,
) {
    let animation = animation_map
        .0
        .get(&AnimationKey::FireSlash)
        .expect("Fire Slash animation animation were not found");

    commands.spawn((
        Sprite::from_atlas_image(
            animation.texture.clone(),
            TextureAtlas {
                layout: animation.atlas.clone(),
                index: 1,
            },
        ),
        EntityState::Idle,
        Transform::from_xyz(
            player_transform.translation.x,
            player_transform.translation.y - PLAYER_LENGTH + 2.0,
            player_transform.translation.z + 1.,
        ),
        FireSlash,
        Damage(DAMAGE),
        Collider::rectangle(FIRE_SLASH_WIDTH, FIRE_SLASH_HEIGHT),
        Friendly,
        Sensor,
    ));
}

fn animate_then_despawn(
    mut commands: Commands,
    map: Res<AnimationMap>,
    mut query: Query<(Entity, &mut Sprite, &EntityState), With<FireSlash>>,
) {
    for (entity, mut sprite, state) in query.iter_mut() {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            let fire_slash_animation = map
                .0
                .get(&AnimationKey::FireSlash)
                .expect("Animation were not found");
            let frames = fire_slash_animation
                .indices
                .get(state)
                .unwrap_or(&fire_slash_animation.default);

            atlas.index += 1;
            if atlas.index >= frames.last_frame {
                commands.entity(entity).despawn();
            }
        }
    }
}
