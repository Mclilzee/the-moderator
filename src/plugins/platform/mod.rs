use crate::bundles::platforms::Platforms;
use bevy::prelude::*;
use bevy::color::palettes::css::RED;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands) {
    commands.spawn(Platforms::cuboid(
        RED.into(),
        Vec2::new(800.0, 20.0),
        Transform::from_xyz(-100.0, -10.0, 0.0),
    ));
}
