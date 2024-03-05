use bevy::prelude::*;

use crate::bundles::platforms::Platform;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands) {
    commands.spawn(Platform::new(Color::BLUE, Vec2::new(1000.0, 200.0)));
}
