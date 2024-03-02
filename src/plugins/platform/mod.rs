use bevy::prelude::*;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::PURPLE,
            custom_size: Some(Vec2::new(200.0, 50.0)),
            ..default()
        },
        ..default()
    });
}
