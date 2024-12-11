use std::time::Duration;

use crate::{
    common_components::{Damage, Enemy, EntityState, Friendly, Health},
    plugins::{
        asset_loader::{AnimationEvent, AnimationKey, AnimationMap},
        player::{Player, PLAYER_LENGTH},
    },
};

use avian2d::prelude::{Collider, Sensor};
use bevy::prelude::*;

const FIRE_SLASH_WIDTH: f32 = 80.0;
const FIRE_SLASH_HEIGHT: f32 = 10.0;
const DAMAGE: i32 = 10;

#[derive(Component)]
struct FireSlash;

#[derive(Resource, Deref, DerefMut)]
struct Cooldown(Timer);

#[derive(Resource, Deref, DerefMut)]
struct DespawnTimer(Timer);

pub struct FireSlashPlugin;

impl Plugin for FireSlashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn, collision).chain())
            .add_systems(
                Update,
                animate_then_despawn.run_if(on_event::<AnimationEvent>),
            )
            .add_systems(Update, despawn);
    }
}

fn spawn(
    mut commands: Commands,
    player: Query<(&Transform, &EntityState), With<Player>>,
    animation_map: Res<AnimationMap>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (p_transform, p_state) = player.single();
    if keys.any_just_pressed([KeyCode::ArrowUp, KeyCode::Space]) && *p_state != EntityState::DoubleJumping {
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
                p_transform.translation.x,
                p_transform.translation.y - PLAYER_LENGTH + 2.0,
                p_transform.translation.z + 1.,
            ),
            FireSlash,
            Damage(DAMAGE),
            Collider::rectangle(FIRE_SLASH_WIDTH, FIRE_SLASH_HEIGHT),
            Friendly,
            Sensor,
        ));
    }
}

fn despawn(
    mut commands: Commands,
    hammers: Query<Entity, With<FireSlash>>,
    mut despawn_timer: ResMut<DespawnTimer>,
    time: Res<Time>,
) {
    despawn_timer.tick(time.delta());
    if despawn_timer.finished() {
        hammers.iter().for_each(|id| {
            if let Some(mut entity) = commands.get_entity(id) {
                entity.despawn();
            }
        });
    }
}

fn collision(
    mut hammers: Query<(Entity, &Damage), (With<FireSlash>, With<Collider>)>,
    mut enemies: Query<(Entity, &mut Health), (Without<FireSlash>, With<Enemy>, With<Collider>)>,
) {
    //for (h_id, h_dmg) in hammers.iter_mut() {
    //    for (e_id, mut e_hp) in enemies.iter_mut() {
    //        if rapier_context.intersection_pair(h_id, e_id).is_some() {
    //            e_hp.0 -= h_dmg.0;
    //        }
    //    }
    //}
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
