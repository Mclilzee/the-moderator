use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;

#[derive(Default, Component)]
pub struct Platform;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_platforms);
    }
}

fn spawn_platforms(mut commands: Commands) {
    commands.spawn((
        Platform,
        Collider::cuboid(8000.0, 20.0),
    ));
}
