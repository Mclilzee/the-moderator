mod bundles;
mod common_components;
mod plugins;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use plugins::{asset_loader, camera_plugin, default_plugins, enemies, platform, player};

#[derive(Clone, PartialEq, Eq, Debug, Hash, SystemSet)]
pub enum InGameSet {
    Input,
    Play,
}

fn main() {
    App::new()
        .configure_sets(Update, (InGameSet::Input, InGameSet::Play))
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
