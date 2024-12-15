use crate::{
    bundles::actors::Actor,
    common_components::{CollisionLayer, Damage, Enemy},
    plugins::{
        asset_loader::AnimationEvent,
        player::Player,
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
const SPAMMER_HP: i32 = 10;
const SPAMMER_DAMAGE: i32 = 1;
const SPAMMER_SPEED: f32 = 40.0;
const SPAMMER_RADIUS: f32 = 10.0;
const SPAMMER_LENGTH: f32 = 5.0;
const SPAMMER_LIMIT: usize = 5;

#[derive(Component)]
struct Spammer;

#[derive(Resource)]
struct SpammerSpawnTimer {
    timer: Timer,
}

pub struct SpammerPlugin;

impl Plugin for SpammerPlugin {
    fn build(&self, app: &mut App) {
        let timer = SpammerSpawnTimer {
            timer: Timer::from_seconds(SPAMMER_SPAWN_TIMER, TimerMode::Repeating),
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

        let animation = asset_loader
            .0
            .get(&AnimationKey::Spammer)
            .expect("Spammer animation were not found");

        commands.spawn((
            Spammer,
            Sprite::from_atlas_image(
                animation.texture.clone(),
                TextureAtlas {
                    layout: animation.atlas.clone(),
                    index: 1,
                },
            ),
            Transform::from_translation(player_translation + Vec3::new(offset, 0.0, 0.0)),
            Actor::new(SPAMMER_HP, SPAMMER_RADIUS, SPAMMER_LENGTH),
            Damage(SPAMMER_DAMAGE),
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

fn animate_spammer(
    mut query: Query<(&mut Sprite, &EntityState), With<Spammer>>,
    map: Res<AnimationMap>,
) {
    query.iter_mut().for_each(|(mut sprite, state)| {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            animate(atlas, state, &AnimationKey::Spammer, &map);
        }
    });
}
