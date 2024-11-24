use crate::{
    bundles::actors::Actor,
    common_components::{DespawnTimer, Health},
    AnimationTimer,
};
use crate::{
    common_components::EntityState,
    plugins::asset_loader::{AnimationKey, AnimationMap},
};
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::{LockedAxes, Velocity},
    geometry::{ActiveEvents, CollisionGroups, Group},
};
use rand::Rng;

use super::player::Player;

const SPAMMER_STARTING_HP: i32 = 20;
const SPAMMER_SPEED: f32 = 40.0;
const SPAMMER_WIDTH: f32 = 10.0;
const SPAMMER_HEIGHT: f32 = 15.0;
const SPAMMER_LIMIT: usize = 5;
const DEATH_EFFECT_DURATION: f32 = 1.0;

pub struct SpammerPlugins;

#[derive(Resource)]
struct SpammerSpawnTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Spammer;

#[derive(Component)]
struct SpammerDespawnEffect;

impl Plugin for SpammerPlugins {
    fn build(&self, app: &mut App) {
        let timer = SpammerSpawnTimer {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        };
        app.insert_resource(timer)
            .add_systems(Update, spawn_spammer)
            .add_systems(Update, track_player)
            .add_systems(Update, animate)
            .add_systems(Update, despawn)
            .add_systems(Update, despawn_effect_progress);
    }
}

fn spawn_spammer(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpammerSpawnTimer>,
    spammers_query: Query<&Spammer>,
    time: Res<Time>,
    camera_query: Query<&OrthographicProjection, (With<Camera>, Without<Player>)>,
    asset_loader: Res<AnimationMap>,
) {
    if spammers_query.iter().count() > SPAMMER_LIMIT {
        return;
    }

    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.just_finished() {
        let camera = camera_query.single();
        let mut random = rand::thread_rng();
        let offset = random.gen_range(-50.0..50.0);
        let camera_offset = camera.area.width() / 2.0;
        let spawn_x = offset + f32::copysign(camera_offset + 5.0, offset);

        let mut spammer = Actor::new(SPAMMER_STARTING_HP, SPAMMER_WIDTH, SPAMMER_HEIGHT);

        let animation = asset_loader
            .0
            .get(&AnimationKey::Spammer)
            .expect("Spammer animation were not found");

        spammer.sprite_bundle.texture = animation.texture.clone();
        spammer.atlas = TextureAtlas {
            layout: animation.atlas.clone(),
            index: 1,
        };

        spammer.sprite_bundle.transform.translation = Vec3::new(spawn_x, 0.0, 0.0);

        commands.spawn((
            spammer,
            Spammer,
            EntityState::Idle,
            CollisionGroups::new(Group::GROUP_2, Group::GROUP_1),
            LockedAxes::ROTATION_LOCKED,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

type WithSpammer = (With<Spammer>, Without<Player>);
type WithPlayer = (With<Player>, Without<Spammer>);

fn track_player(
    mut spammer_query: Query<(&Transform, &mut Velocity), WithSpammer>,
    player_query: Query<&Transform, WithPlayer>,
) {
    let player_transform = player_query.single();

    for (transform, mut velocity) in spammer_query.iter_mut() {
        velocity.linvel.x = if player_transform.translation.x > transform.translation.x {
            SPAMMER_SPEED
        } else {
            -SPAMMER_SPEED
        };
    }
}

fn animate(
    mut sprite_query: Query<
        (&mut TextureAtlas, &mut Sprite, &EntityState, &Velocity),
        With<Spammer>,
    >,
    animation_timer: Res<AnimationTimer>,
    animation: Res<AnimationMap>,
) {
    if !animation_timer.0.finished() {
        return;
    }

    let spammer_animations = &animation
        .0
        .get(&AnimationKey::Spammer)
        .expect("Animation for spammer were not found");

    for (mut atlas, mut sprite, state, velocity) in sprite_query.iter_mut() {
        let frames = spammer_animations
            .indices
            .get(state)
            .unwrap_or(&spammer_animations.default);

        if velocity.linvel.x < 0.0 {
            sprite.flip_x = true;
        } else if velocity.linvel.x > 0.0 {
            sprite.flip_x = false;
        }

        let mut index = atlas.index + 1;

        if atlas.index >= frames.last_frame || atlas.index < frames.first_frame {
            index = frames.first_frame;
        }

        atlas.index = index;
    }
}

fn despawn(mut commands: Commands, query: Query<(Entity, &Health, &Transform), With<Spammer>>) {
    for (id, hp, transform) in query.iter() {
        if hp.0 <= 0 {
            commands.entity(id).despawn();
            commands.spawn((
                SpammerDespawnEffect,
                DespawnTimer(Timer::from_seconds(DEATH_EFFECT_DURATION, TimerMode::Once)),
                SpriteBundle {
                    transform: *transform,
                    ..default()
                },
            ));
        }
    }
}

fn despawn_effect_progress(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnTimer, &mut Sprite), With<SpammerDespawnEffect>>,
    time: Res<Time>,
) {
    for (id, mut stopwatch, mut sprite) in query.iter_mut() {
        stopwatch.0.tick(time.delta());
        let size = stopwatch.0.elapsed_secs() * 20.0;
        sprite.custom_size = Some(Vec2::new(size, size));
        if stopwatch.0.finished() {
            commands.entity(id).despawn();
        }
    }
}
