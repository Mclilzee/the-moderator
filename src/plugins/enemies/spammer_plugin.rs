use crate::{
    bundles::actors::Actor,
    common_components::{CollisionLayer, Damage, DespawnTimer, Enemy, Health},
    plugins::{
        asset_loader::AnimationEvent,
        player::{Player, ScoreUpdateEvent},
    },
    utils::animate,
};
use crate::{
    common_components::EntityState,
    plugins::asset_loader::{AnimationKey, AnimationMap},
};
use avian2d::prelude::{CollisionLayers, LinearVelocity, LockedAxes};
use bevy::prelude::*;
use rand::Rng;

const SPAMMER_SPAWN_TIMER: f32 = 0.2;
const SPAMMER_STARTING_HP: i32 = 20;
const SPAMMER_DAMAGE: i32 = 1;
const SPAMMER_SPEED: f32 = 40.0;
const SPAMMER_WIDTH: f32 = 10.0;
const SPAMMER_HEIGHT: f32 = 15.0;
const SPAMMER_LIMIT: usize = 5;
const POINTS_INCREMENT_DURATION: f32 = 1.0;
const POINTS_INCREMENT_ASCENDING_SPEED: f32 = 200.0;
const POINTS_SIZE: f32 = 20.0;
const POINTS_REWARDED: u32 = 1;

#[derive(Component)]
struct Spammer;

#[derive(Resource)]
struct SpammerSpawnTimer {
    timer: Timer,
}

#[derive(Component)]
struct PointsIncrementEffect;

pub struct SpammerPlugin;

impl Plugin for SpammerPlugin {
    fn build(&self, app: &mut App) {
        let timer = SpammerSpawnTimer {
            timer: Timer::from_seconds(SPAMMER_SPAWN_TIMER, TimerMode::Repeating),
        };

        app.insert_resource(timer)
            .add_systems(Update, animate_spammer.run_if(on_event::<AnimationEvent>()))
            .add_systems(Update, spawn_spammer)
            .add_systems(Update, track_player)
            .add_systems(Update, flip_on_movement)
            .add_systems(Update, despawn)
            .add_systems(Update, despawn_points_increment_effect);
    }
}

fn spawn_spammer(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpammerSpawnTimer>,
    spammers_query: Query<&Spammer>,
    time: Res<Time>,
    camera_query: Query<&OrthographicProjection, With<Camera>>,
    player_query: Query<&Transform, With<Player>>,
    asset_loader: Res<AnimationMap>,
) {
    if spammers_query.iter().count() > SPAMMER_LIMIT {
        return;
    }

    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.just_finished() {
        let camera = camera_query.single();
        let player_translation = player_query.single().translation;
        let mut random = rand::thread_rng();
        let offset = random.gen_range(-50.0..50.0);
        let offset = ((camera.area.width() / 2.0) + 20.0).copysign(offset);
        let spammer_translation = player_translation + Vec3::new(offset, 0.0, 0.0);
        let mut actor = Actor::new(SPAMMER_STARTING_HP, SPAMMER_WIDTH, SPAMMER_HEIGHT);

        let animation = asset_loader
            .0
            .get(&AnimationKey::Spammer)
            .expect("Spammer animation were not found");

        actor.sprite_bundle.texture = animation.texture.clone();
        actor.atlas = TextureAtlas {
            layout: animation.atlas.clone(),
            index: 1,
        };
        actor.sprite_bundle.transform.translation = spammer_translation;

        commands.spawn((
            actor,
            Spammer,
            Damage(SPAMMER_DAMAGE),
            Enemy,
            EntityState::default(),
            CollisionLayers::new(CollisionLayer::Enemy, [CollisionLayer::Friendly, CollisionLayer::Wall]),
            LockedAxes::ROTATION_LOCKED,
            //ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

type WithSpammer = (With<Spammer>, Without<Player>);
type WithPlayer = (With<Player>, Without<Spammer>);

fn track_player(
    mut spammer_query: Query<(&Transform, &mut LinearVelocity), WithSpammer>,
    player_query: Query<&Transform, WithPlayer>,
) {
    let player_transform = player_query.single();

    for (transform, mut velocity) in spammer_query.iter_mut() {
        velocity.x = if player_transform.translation.x > transform.translation.x {
            SPAMMER_SPEED
        } else {
            -SPAMMER_SPEED
        };
    }
}

fn flip_on_movement(mut spammers: Query<(&mut Sprite, &LinearVelocity), With<Spammer>>) {
    for (mut sprite, velocity) in spammers.iter_mut() {
        if velocity.x < 0.0 {
            sprite.flip_x = true;
        } else if velocity.x > 0.0 {
            sprite.flip_x = false;
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<(Entity, &Health, &Transform), With<Spammer>>,
    mut event: EventWriter<ScoreUpdateEvent>,
) {
    query.iter().for_each(|(id, hp, transform)| {
        if hp.0 <= 0 {
            commands.entity(id).despawn();
            commands.spawn((
                PointsIncrementEffect,
                DespawnTimer(Timer::from_seconds(
                    POINTS_INCREMENT_DURATION,
                    TimerMode::Once,
                )),
                Text2dBundle {
                    text: Text::from_section(
                        "++",
                        TextStyle {
                            font_size: POINTS_SIZE,
                            ..default()
                        },
                    ),
                    transform: *transform,
                    ..default()
                },
            ));

            event.send(ScoreUpdateEvent {
                gained_points: POINTS_REWARDED,
            });
        }
    });
}

fn despawn_points_increment_effect(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DespawnTimer, &mut Transform), With<PointsIncrementEffect>>,
    time: Res<Time>,
) {
    for (id, mut timer, mut transform) in query.iter_mut() {
        timer.0.tick(time.delta());
        transform.translation.y += POINTS_INCREMENT_ASCENDING_SPEED * time.delta_seconds();
        if timer.0.finished() {
            commands.entity(id).despawn();
        }
    }
}

fn animate_spammer(
    mut query: Query<(&mut TextureAtlas, &EntityState), With<Spammer>>,
    map: Res<AnimationMap>,
) {
    query.iter_mut().for_each(|(mut atlas, state)| {
        animate(&mut atlas, state, &AnimationKey::Spammer, &map);
    });
}
