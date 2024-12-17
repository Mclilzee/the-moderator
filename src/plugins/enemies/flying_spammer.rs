use crate::{
    common_components::{CollisionLayer, Damage, Enemy, Health},
    plugins::{
        asset_loader::AnimationEvent,
        player::{Player, Score},
    },
    utils::animate,
};
use crate::{
    common_components::{EntityState, Projectile},
    plugins::asset_loader::{AnimationKey, AnimationMap},
};
use avian2d::prelude::{AngularVelocity, Collider, CollisionLayers, LinearVelocity, RigidBody, Sensor};
use bevy::prelude::*;

const FLYING_SPAMMER_SPAWN_TIMER: f32 = 0.2;
const FLYING_SPAMMER_HP: i32 = 10;
const FLYING_SPAMMER_DAMAGE: i32 = 5;
const FLYING_SPAMMER_SPEED: f32 = 50.0;
const FLYING_SPAMMER_RADIUS: f32 = 10.0;
const FLYING_SPAMMER_LENGTH: f32 = 5.0;
const FLYING_Y_DISTANCE_FROM_PLAYER: f32 = 200.0;
const FLYING_X_DISTANCE_FROM_PLAYER: f32 = 200.0;
const PEACH_SHOOTING_COOLDOWN: f32 = 4.0;
const PEACH_SIZE: f32 = 16.0;
const PEACH_SPEED: f32 = 200.0;
const PEACH_ROATION_ANGLE: f32 = 10.0;

#[derive(Component)]
struct FlyingSpammer;

#[derive(Component)]
struct PeachCooldown(Timer);

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
            .add_systems(Update, shoot_peach)
            .add_systems(Update, flip_on_movement);
    }
}

fn spawn_spammer(
    mut commands: Commands,
    mut spawn_timer: ResMut<FlyingSpammerSpawnTimer>,
    spammers_query: Query<&FlyingSpammer>,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    asset_loader: Res<AnimationMap>,
    player_score: Res<Score>,
) {
    if spammers_query.iter().count() > player_score.0 as usize / 20 {
        return;
    }

    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.just_finished() {
        let player_translation = player_query.single().translation;

        let animation = asset_loader
            .0
            .get(&AnimationKey::FlyingSpammer)
            .expect("Flying Spammer animation were not found");

        commands.spawn((
            FlyingSpammer,
            PeachCooldown(Timer::from_seconds(
                PEACH_SHOOTING_COOLDOWN,
                TimerMode::Repeating,
            )),
            Sprite::from_atlas_image(
                animation.texture.clone(),
                TextureAtlas {
                    layout: animation.atlas.clone(),
                    index: 1,
                },
            ),
            Transform::from_translation(
                player_translation + Vec3::new(0.0, FLYING_Y_DISTANCE_FROM_PLAYER + 300., 0.0),
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
        ));
    }
}

type WithFlyingSpammer = (With<FlyingSpammer>, Without<Player>);
type WithPlayer = (With<Player>, Without<FlyingSpammer>);

fn track_player(
    mut spammer_query: Query<(&Transform, &mut LinearVelocity), WithFlyingSpammer>,
    player_query: Query<&Transform, WithPlayer>,
) {
    let player_transform = player_query.single();

    for (spammer_transform, mut spammer_velocity) in spammer_query.iter_mut() {
        spammer_velocity.x = if player_transform.translation.x
            > spammer_transform.translation.x + FLYING_Y_DISTANCE_FROM_PLAYER
        {
            FLYING_SPAMMER_SPEED
        } else if player_transform.translation.x
            < spammer_transform.translation.x - FLYING_Y_DISTANCE_FROM_PLAYER
        {
            -FLYING_SPAMMER_SPEED
        } else {
            0.
        };

        spammer_velocity.y = if spammer_transform.translation.y
            > player_transform.translation.y + FLYING_X_DISTANCE_FROM_PLAYER
        {
            -FLYING_SPAMMER_SPEED
        } else {
            0.
        };
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

fn shoot_peach(
    mut commands: Commands,
    mut flying_spammer_q: Query<
        (&mut PeachCooldown, &LinearVelocity, &Transform),
        WithFlyingSpammer,
    >,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    player: Single<&Transform, WithPlayer>,
) {
    for (mut cooldown, velocity, transform) in flying_spammer_q.iter_mut() {
        cooldown.0.tick(time.delta());
        if velocity.0 != Vec2::ZERO {
            continue;
        }

        if cooldown.0.finished() {
            let p1 = transform.translation.truncate();
            let p2 = player.translation.truncate();

            let l_velocity = LinearVelocity::from((p2 - p1).normalize() * PEACH_SPEED);
            let a_velocity = AngularVelocity::from(if l_velocity.x >= 0.0 {
                -PEACH_ROATION_ANGLE
            } else {
                PEACH_ROATION_ANGLE
            });

            commands.spawn((
                Sprite::from_image(asset_server.load("peach.png")),
                Projectile::default(),
                Collider::circle(PEACH_SIZE),
                Enemy,
                RigidBody::Kinematic,
                Sensor,
                CollisionLayers::new(
                    CollisionLayer::Enemy,
                    [CollisionLayer::Friendly, CollisionLayer::Wall],
                ),
                l_velocity,
                a_velocity,
                *transform,
            ));
        }
    }
}
