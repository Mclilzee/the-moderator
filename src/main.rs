mod bundles;
mod common_components;
mod plugins;

mod utils;
pub struct WorldBoundry {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

pub const WORLD_BOUNDRY: WorldBoundry = WorldBoundry {
    left: 645.,
    right: 4561.,
    top: 0.,
    bottom: 0.,
};

use avian2d::{prelude::Gravity, PhysicsPlugins};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use plugins::{asset_loader, camera_plugin, collisions, default_plugins, enemies, player, walls};

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
        .add_plugins(enemies::EnemiesPlugin)
        .add_plugins(collisions::CollisionsHandlerPlugin);

    //app.add_plugins(PhysicsDebugPlugin::default());
    app.run();
}
