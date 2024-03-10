use crate::{
    bundles::actors::Actor,
    components::{Grounded, Player, Spammer, Velocity},
};
use crate::{
    components::EntityState,
    plugins::asset_loader::{AnimationKey, AnimationMap},
    AnimationTimer,
};
use bevy::prelude::*;
use rand::Rng;

const SPAMMER_STARTING_HP: i32 = 20;
const SPAMMER_SPEED: f32 = 40.0;
const SPAMMER_WIDTH: f32 = 25.0;
const SPAMMER_HEIGHT: f32 = 30.0;
const SPAMMER_LIMIT: usize = 100;

pub struct SpammerPlugins;

#[derive(Resource)]
struct SpammerSpawnTimer {
    timer: Timer,
}

impl Plugin for SpammerPlugins {
    fn build(&self, app: &mut App) {
        let timer = SpammerSpawnTimer {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        };
        app.insert_resource(timer)
            .add_systems(Update, spawn_spammer)
            .add_systems(Update, track_player)
            .add_systems(Update, animate);
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

        let mut spammer = Actor::new(
            SPAMMER_STARTING_HP,
            Vec2::new(SPAMMER_WIDTH, SPAMMER_HEIGHT),
        );

        let animation = asset_loader
            .0
            .get(&AnimationKey::Spammer)
            .expect("Spammer animation were not found");

        spammer.movable_sprite.sprite_sheet.texture = animation.texture.clone();
        spammer.movable_sprite.sprite_sheet.atlas = TextureAtlas {
            layout: animation.atlas.clone(),
            index: 1,
        };

        spammer.movable_sprite.sprite_sheet.transform.translation = Vec3::new(spawn_x, 0.0, 0.0);

        commands.spawn((spammer, Spammer, Grounded(true), EntityState::Idle));
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
        velocity.0.x = if player_transform.translation.x > transform.translation.x {
            SPAMMER_SPEED
        } else {
            -SPAMMER_SPEED
        };
    }
}

pub fn animate(
    mut sprite_query: Query<
        (&mut TextureAtlas, &mut Sprite, &EntityState, &Velocity),
        With<Spammer>,
    >,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
    animation: Res<AnimationMap>,
) {
    timer.0.tick(time.delta());
    if !timer.0.finished() {
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

        if velocity.0.x < 0.0 {
            sprite.flip_x = true;
        } else if velocity.0.x > 0.0 {
            sprite.flip_x = false;
        }

        let mut index = atlas.index + 1;

        if atlas.index >= frames.last_frame || atlas.index < frames.first_frame {
            index = frames.first_frame;
        }

        atlas.index = index;
    }
}
