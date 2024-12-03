mod bundles;
mod common_components;
mod plugins;
mod utils;

use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use plugins::{asset_loader, camera_plugin, default_plugins, enemies, player, walls};

#[derive(Clone, PartialEq, Eq, Debug, Hash, SystemSet)]
pub enum InGameSet {
    Input,
    Play,
}

fn main() {
    let mut app = App::new();

    app.configure_sets(Update, (InGameSet::Input, InGameSet::Play))
        .add_plugins(default_plugins::CustomDefaultPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(LdtkPlugin)
        .add_plugins(camera_plugin::CameraPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemies::EnemiesPlugin);

    //#[cfg(dev)]
    app.run();
}
