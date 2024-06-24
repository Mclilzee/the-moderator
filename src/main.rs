mod bundles;
mod components;
mod plugins;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use plugins::{
    asset_loader::AssetLoaderPlugin, default_plugins, mouse, platform, player, spammer_plugin,
};
use std::time::Duration;

#[derive(Clone, PartialEq, Eq, Debug, Hash, SystemSet)]
pub enum InGameSet {
    Input,
    Play,
}

#[derive(Resource)]
pub struct AnimationTimer(pub Timer);

fn main() {
    App::new()
        .configure_sets(Update, (InGameSet::Input, InGameSet::Play))
        .insert_resource(AnimationTimer(Timer::new(
            Duration::from_millis(100),
            TimerMode::Repeating,
        )))
        .add_plugins(default_plugins::CustomDefaultPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(platform::PlatformPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(spammer_plugin::SpammerPlugins)
        .add_plugins(mouse::MousePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
