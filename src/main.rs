mod bundles;
mod components;
mod consts;
mod debugging;
mod plugins;

use std::time::Duration;

use bevy::{prelude::*, render::camera::ScalingMode};
use components::Player;
use debugging::debug_boxes::DebugBoxPlugin;
use plugins::{animation_loader::AnimationLoaderPlugin, default_plugins, player, spammer_plugin};

#[derive(Clone, PartialEq, Eq, Debug, Hash, SystemSet)]
pub enum InGameSet {
    Movement,
    UserInput,
}

#[derive(Resource)]
pub struct AnimationTimer(pub Timer);

fn main() {
    App::new()
        .configure_sets(Update, (InGameSet::UserInput, InGameSet::Movement))
        .insert_resource(AnimationTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Repeating,
        )))
        .add_plugins(default_plugins::CustomDefaultPlugin)
        .add_plugins(AnimationLoaderPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(spammer_plugin::SpammerPlugins)
        .add_plugins(DebugBoxPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(PostUpdate, follow_player)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 200.0,
        min_height: 100.0,
    };

    commands.spawn(camera);
}

fn follow_player(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let mut translation = player_query.single().translation;
    translation.y += 40.0;

    camera_transform.translation = translation;
}
