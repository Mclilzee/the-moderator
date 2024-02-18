use crate::{
    bundles::{character::Character, movable_object::MovableObject},
    components::{Hp, Player, Spammer, Velocity},
};
use bevy::{asset::AssetLoader, prelude::*, window::PrimaryWindow};
use rand::{self, Rng};

use super::assets_plugin::{AnimationType, AssetsLoader};

const SPAMMER_STARTING_HP: i32 = 5;
const SPAMMER_SPEED: f32 = 60.0;
const SPAMMER_WIDTH: f32 = 25.0;
const SPAMMER_HEIGHT: f32 = 40.0;

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
            .add_systems(Update, track_player);
    }
}

fn spawn_spammer(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpammerSpawnTimer>,
    spammers_query: Query<&Spammer>,
    time: Res<Time>,
    camera_query: Query<&OrthographicProjection, (With<Camera>, Without<Player>)>,
) {
    // Limit spawn to 10 spammers
    if spammers_query.iter().count() > 10 {
        return;
    }

    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.just_finished() {
        let camera = camera_query.single();
        let mut random = rand::thread_rng();
        let offset = random.gen_range(-50.0..50.0);
        let camera_offset = camera.area.width() / 2.0;

        let spawn_x = offset + f32::copysign(camera_offset + 5.0, offset);

        let mut spammer = Character::new(SPAMMER_STARTING_HP, Vec2::splat(32.0));
        spammer.movable_object.sprite_sheet.transform.translation = Vec3::new(spawn_x, 0.0, 0.0);

        commands.spawn((spammer, Spammer));
    }
}

type WithSpammer = (With<Spammer>, Without<Player>);
type WithPlayer = (With<Player>, Without<Spammer>);

fn track_player(
    mut spammer_query: Query<&mut Transform, WithSpammer>,
    player_query: Query<&Transform, WithPlayer>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();

    for mut transform in spammer_query.iter_mut() {
        let velocity = if player_transform.translation.x > transform.translation.x {
            Vec3::new(SPAMMER_SPEED, 0.0, 0.0)
        } else {
            Vec3::new(-SPAMMER_SPEED, 0.0, 0.0)
        };

        transform.translation += velocity * time.delta_seconds();
    }
}
