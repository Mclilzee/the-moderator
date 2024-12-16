use crate::{
    common_components::EntityState,
    plugins::asset_loader::{AnimationKey, AnimationMap},
};
use crate::{
    common_components::{CollisionLayer, Damage, Enemy, Health},
    plugins::{
        asset_loader::AnimationEvent,
        player::{Player, Score},
    },
    utils::animate,
};
use avian2d::prelude::{Collider, CollisionLayers, LinearVelocity, LockedAxes, RigidBody};
use bevy::prelude::*;
use rand::Rng;

const FLYING_SPAMMER_SPAWN_TIMER: f32 = 0.2;
const FLYING_SPAMMER_HP: i32 = 10;
const FLYING_SPAMMER_DAMAGE: i32 = 5;
const FLYING_SPAMMER_SPEED: f32 = 100.0;
const FLYING_SPAMMER_RADIUS: f32 = 10.0;
const FLYING_SPAMMER_LENGTH: f32 = 5.0;
const FLYING_OFFSET_FROM_PLAYER: f32 = 20.0;

#[derive(Component)]
struct FlyingSpammer;

#[derive(Resource)]
struct FlyingSpammerSpawnTimer {
    timer: Timer,
}

pub struct FlyingSpammerPlugin;

impl Plugin for FlyingSpammerPlugin {
    fn build(&self, app: &mut App) {
        let timer = FlyingSpammerSpawnTimer {
            timer: Timer::from_seconds(FLYING_SPAMMER_SPAWN_TIMER, TimerMode::Repeating),
        };

        app.insert_resource(timer)
            .add_systems(Update, animate_spammer.run_if(on_event::<AnimationEvent>))
            .add_systems(Update, spawn_spammer)
            .add_systems(Update, track_player)
            .add_systems(Update, flip_on_movement);
    }
}

fn spawn_spammer(
    mut commands: Commands,
    mut spawn_timer: ResMut<FlyingSpammerSpawnTimer>,
    spammers_query: Query<&FlyingSpammer>,
    time: Res<Time>,
    camera_query: Query<&OrthographicProjection, With<Camera>>,
    player_query: Query<&Transform, With<Player>>,
    asset_loader: Res<AnimationMap>,
    player_score: Res<Score>,
) {
    if spammers_query.iter().count() > player_score.0 as usize / 20 {
        return;
    }

    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.just_finished() {
        let camera = camera_query.single();
        let player_translation = player_query.single().translation;

        let animation = asset_loader
            .0
            .get(&AnimationKey::FlyingSpammer)
            .expect("Flying Spammer animation were not found");

        commands.spawn((
            FlyingSpammer,
            Sprite::from_atlas_image(
                animation.texture.clone(),
                TextureAtlas {
                    layout: animation.atlas.clone(),
                    index: 1,
                },
            ),
            Transform::from_translation(
                player_translation + Vec3::new(0.0, camera.area.height() + 10.0, 0.0),
            ),
            RigidBody::Kinematic,
            Collider::capsule(FLYING_SPAMMER_RADIUS, FLYING_SPAMMER_LENGTH),
            Health(FLYING_SPAMMER_HP),
            Damage(FLYING_SPAMMER_DAMAGE),
            Enemy,
            EntityState::default(),
            CollisionLayers::new(
                CollisionLayer::Enemy,
                [CollisionLayer::Friendly, CollisionLayer::Wall],
            ),
            LockedAxes::ROTATION_LOCKED,
        ));
    }
}

type WithFlyingSpammer = (With<FlyingSpammer>, Without<Player>);
type WithPlayer = (With<Player>, Without<FlyingSpammer>);

fn track_player(
    mut spammer_query: Query<(&Transform, &mut LinearVelocity), WithFlyingSpammer>,
    player_query: Query<&Transform, WithPlayer>,
    camera_query: Query<&OrthographicProjection, With<Camera>>,
) {
    let player_transform = player_query.single();
    let camera = camera_query.single();

    for (transform, mut velocity) in spammer_query.iter_mut() {
        velocity.x = if player_transform.translation.x
            > transform.translation.x + FLYING_OFFSET_FROM_PLAYER
        {
            FLYING_SPAMMER_SPEED
        } else if player_transform.translation.x
            < transform.translation.x - FLYING_OFFSET_FROM_PLAYER
        {
            -FLYING_SPAMMER_SPEED
        } else {
            0.
        };

        if player_transform.translation.y > transform.translation.x + camera.area.height() / 2.0 {
            velocity.y = -FLYING_SPAMMER_SPEED
        }
    }
}

fn flip_on_movement(mut spammers: Query<(&mut Sprite, &LinearVelocity), With<FlyingSpammer>>) {
    for (mut sprite, velocity) in spammers.iter_mut() {
        if velocity.x < 0.0 {
            sprite.flip_x = true;
        } else if velocity.x > 0.0 {
            sprite.flip_x = false;
        }
    }
}

fn animate_spammer(
    mut query: Query<(&mut Sprite, &EntityState), With<FlyingSpammer>>,
    map: Res<AnimationMap>,
) {
    query.iter_mut().for_each(|(mut sprite, state)| {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            animate(atlas, state, &AnimationKey::FlyingSpammer, &map);
        }
    });
}
