mod bundles;
mod common_components;
mod plugins;
mod utils;

use avian2d::{prelude::{Gravity, PhysicsDebugPlugin}, PhysicsPlugins};
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
        .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .insert_resource(Gravity(Vec2::NEG_Y * 1000.0))
        .add_plugins(LdtkPlugin)
        .add_plugins(camera_plugin::CameraPlugin)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(walls::WallPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(enemies::EnemiesPlugin);

    //#[cfg(dev)]
    app.add_plugins(PhysicsDebugPlugin::default());
    app.run();
}
