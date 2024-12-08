use bevy::{prelude::*, winit::cursor::{CursorIcon, CustomCursor}};

pub struct CustomDefaultPlugin;

#[derive(Resource)]
pub struct CursorPosition(pub Vec2);

impl Plugin for CustomDefaultPlugin {
    fn build(&self, app: &mut App) {
        let window = Window {
            title: "The Moderator: Fred's Revenge".to_string(),
            resolution: (1920.0, 1080.0).into(),
            resizable: true,
            ..default()
        };

        let default_plugins = DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            })
            .build();

        app.add_plugins(default_plugins).add_systems(PostStartup, insert_cursor);
    }
}

fn insert_cursor(mut commands: Commands, window: Single<Entity, With<Window>>, asset_server: Res<AssetServer>) {
    commands
        .entity(*window)
        .insert(CursorIcon::Custom(CustomCursor::Image {
            handle: asset_server.load("cursor.png"),
            hotspot: (5, 5),
        }));
}
