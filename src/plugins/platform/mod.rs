use bevy::prelude::*;

use crate::components::Collider;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands) {
    let platform = platform(Vec2::new(1000.0, 200.0));
    commands.spawn(platform);
}

fn platform(size: Vec2) -> (SpriteBundle, Collider) {
    (
        SpriteBundle {
            sprite: Sprite {
                color: Color::PURPLE,
                custom_size: Some(size),
                ..default()
            },
            ..default()
        },
        Collider(size),
    )
}
