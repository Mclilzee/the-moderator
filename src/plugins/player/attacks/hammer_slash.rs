use std::time::Duration;

use crate::{
    common_components::{Damage, Enemy, EntityState, Friendly, Health},
    plugins::{
        asset_loader::{AnimationEvent, AnimationKey, AnimationMap},
        player::Player,
    },
};

use avian2d::prelude::{Collider, Sensor};
use bevy::prelude::*;

const SMASH_WIDTH: f32 = 120.0;
const SMASH_HEIGHT: f32 = 2.0;
const DAMAGE: i32 = 10;
const COOLDOWN_SECS: f32 = 0.5;

#[derive(Component)]
struct FireSlash;

#[derive(Resource, Deref, DerefMut)]
struct Cooldown(Timer);

#[derive(Resource, Deref, DerefMut)]
struct DespawnTimer(Timer);

pub struct FireSlashPlugin;

impl Plugin for FireSlashPlugin {
    fn build(&self, app: &mut App) {
        let mut cooldown = Timer::from_seconds(COOLDOWN_SECS, TimerMode::Once);
        cooldown.tick(Duration::from_secs_f32(COOLDOWN_SECS));

        app.insert_resource(Cooldown(cooldown))
            .add_systems(Update, (spawn, collision).chain())
            .add_systems(Update, animate_then_despawn.run_if(on_event::<AnimationEvent>))
            .add_systems(Update, despawn);
    }
}

fn spawn(
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    animation_map: Res<AnimationMap>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut cooldown: ResMut<Cooldown>,
    time: Res<Time>,
) {
    cooldown.0.tick(time.delta());
    if buttons.just_pressed(MouseButton::Right) && cooldown.0.finished() {
        cooldown.0.reset();

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
                player.translation.x,
                player.translation.y,
                player.translation.z + 1.,
            ),
            FireSlash,
            Damage(DAMAGE),
            Collider::rectangle(SMASH_WIDTH, SMASH_HEIGHT),
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
