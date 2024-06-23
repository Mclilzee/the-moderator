use bevy::prelude::*;
use bevy_rapier2d::geometry::Collider;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands) {
    commands.spawn((
        Collider::cuboid(500.0, 20.0),
        TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)),
    ));
}
