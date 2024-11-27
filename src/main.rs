mod bundles;
mod common_components;
mod plugins;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use plugins::{asset_loader, camera_plugin, default_plugins, enemies, platform, player};
use std::time::Duration;

const DEFAULT_ANIMATION_TIME_MILLIS: u64 = 100;

#[derive(Clone, PartialEq, Eq, Debug, Hash, SystemSet)]
pub enum InGameSet {
    Input,
    Play,
}

#[derive(Resource)]
struct AnimationTimer(pub Timer);

#[derive(Event, Default)]
pub struct AnimationEvent;

fn main() {
    App::new()
        .configure_sets(Update, (InGameSet::Input, InGameSet::Play))
        .insert_resource(AnimationTimer(Timer::new(
            Duration::from_millis(DEFAULT_ANIMATION_TIME_MILLIS),
            TimerMode::Repeating,
        )))
        .add_event::<AnimationEvent>()
        .add_systems(PreUpdate, advance_animation_timer)
        .add_plugins(default_plugins::CustomDefaultPlugin)
        .add_plugins(camera_plugin::CameraPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(platform::PlatformPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemies::EnemiesPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}

fn advance_animation_timer(
    mut timer: ResMut<AnimationTimer>,
    time: Res<Time>,
    mut event: EventWriter<AnimationEvent>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        event.send_default();
    }
}
